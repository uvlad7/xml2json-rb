use std::any::Any;
use std::borrow::Cow;
use std::panic::AssertUnwindSafe;

use robusta_jni::convert::{TryFromJavaValue, TryIntoJavaValue};
use robusta_jni::jni::JNIEnv;
use robusta_jni::jni::sys::jobject;

/// The possible types of [`Error`].
#[derive(Debug)]
pub enum ErrorType {
    /// An error generated in Rust code that will raise an exception when
    /// returned to Java.
    Error(&'static str, Cow<'static, str>),
    /// A Java Exception captured from Java as an Error.
    Exception(robusta_jni::jni::errors::Error),
}

/// Wrapper type for Java `Exception`s or other interrupts.
#[derive(Debug)]
pub struct Error(ErrorType);

impl From<robusta_jni::jni::errors::Error> for Error {
    fn from(val: robusta_jni::jni::errors::Error) -> Self {
        Self(ErrorType::Exception(val))
    }
}

impl Error {
    fn from_panic<'local>(env: &JNIEnv<'local>, e: Box<dyn Any + Send + 'static>) -> ! {
        let msg: Cow<'static, str> = if let Some(&m) = e.downcast_ref::<&'static str>() {
            m.into()
        } else if let Some(m) = e.downcast_ref::<String>() {
            m.clone().into()
        } else {
            "panic".into()
        };
        env.fatal_error(msg);
    }
}

fn raise<T: From<jobject>>(e: Error, env: &JNIEnv) -> T {
    match e.0 {
        ErrorType::Exception(jni_err) => {
            // TODO: Impl ToException trait
            let _ = env.throw_new("java/lang/RuntimeException", format!("{:?}", jni_err));
        }
        ErrorType::Error(cls, msg) => {
            let _ = env.throw_new(cls, msg);
        }
    };
    T::from(std::ptr::null_mut())
}

pub trait ReturnValue<'env, T>
    where
        T: TryIntoJavaValue<'env>,
{
    fn into_return_value(self, env: &JNIEnv<'env>) -> Result<<T as TryIntoJavaValue<'env>>::Target, Error>;
}

impl<'env, T> ReturnValue<'env, T> for Result<T, Error>
    where
        T: TryIntoJavaValue<'env>,
{
    fn into_return_value(self, env: &JNIEnv<'env>) -> Result<<T as TryIntoJavaValue<'env>>::Target, Error> {
        match self {
            Ok(val) => { Ok(TryIntoJavaValue::try_into(val, env)?) }
            Err(err) => { Err(err) }
        }
    }
}

/// Helper trait for wrapping a function as a Java static method ignoring
/// self and taking N arguments, with type conversions and error
/// handling.
///
/// See the [`jni_static_function`](crate::jni_static_function!) macro.
#[doc(hidden)]
pub trait JNIStaticFunction1<'env: 'borrow, 'borrow, Func, T0, Res, Ret>
    where
        Self: Sized + Fn(T0) -> Res,
        T0: TryFromJavaValue<'env, 'borrow>,
        Res: ReturnValue<'env, Ret>,
        Ret: TryIntoJavaValue<'env>,
        <Ret as TryIntoJavaValue<'env>>::Target: From<jobject>,
{
    #[inline]
    fn call_convert_value(self, env: &'borrow JNIEnv<'env>, arg0: T0::Source) -> Result<Ret::Target, Error> {
        let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
        (self)(
            conv_arg0,
        ).into_return_value(env)
    }

    #[inline]
    unsafe fn call_handle_error(self, env: &'borrow JNIEnv<'env>, arg0: T0::Source) -> Ret::Target {
        let res =
            match std::panic::catch_unwind(AssertUnwindSafe(|| {
                self.call_convert_value(env, arg0)
            })) {
                Ok(v) => v,
                Err(e) => Error::from_panic(env, e), // Err(Error::from_panic(env, e)),
            };
        match res {
            Ok(v) => v,
            Err(e) => raise(e, env),
        }
    }
}

impl<'env: 'borrow, 'borrow, Func, T0, Res, Ret> JNIStaticFunction1<'env, 'borrow, Func, T0, Res, Ret> for Func
    where
        Func: Fn(T0) -> Res,
        T0: TryFromJavaValue<'env, 'borrow>,
        Res: ReturnValue<'env, Ret>,
        Ret: TryIntoJavaValue<'env>,
        <Ret as TryIntoJavaValue<'env>>::Target: From<jobject>,
{}

/// Helper trait for wrapping a function as a Java static method
/// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
/// type conversions and error handling.
///
/// See the [`jni_static_function`](crate::jni_static_function!) macro.
#[doc(hidden)]
pub trait JNIEnvStaticFunction1<'env: 'borrow, 'borrow, Func, T0, Res, Ret>
    where
        Self: Sized + Fn(&JNIEnv, T0) -> Res,
        T0: TryFromJavaValue<'env, 'borrow>,
        Res: ReturnValue<'env, Ret>,
        Ret: TryIntoJavaValue<'env>,
        <Ret as TryIntoJavaValue<'env>>::Target: From<jobject>,
{
    #[inline]
    fn call_convert_value(self, env: &'borrow JNIEnv<'env>, arg0: T0::Source) -> Result<Ret::Target, Error> {
        let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
        (self)(
            env,
            conv_arg0,
        ).into_return_value(env)
    }

    #[inline]
    unsafe fn call_handle_error(self, env: &'borrow JNIEnv<'env>, arg0: T0::Source) -> Ret::Target {
        let res =
            match std::panic::catch_unwind(AssertUnwindSafe(|| {
                self.call_convert_value(env, arg0)
            })) {
                Ok(v) => v,
                Err(e) => Error::from_panic(env, e), // Err(Error::from_panic(env, e)),
            };
        match res {
            Ok(v) => v,
            Err(e) => raise(e, env),
        }
    }
}

impl<'env: 'borrow, 'borrow, Func, T0, Res, Ret> JNIEnvStaticFunction1<'env, 'borrow, Func, T0, Res, Ret> for Func
    where
        Func: Fn(&JNIEnv, T0) -> Res,
        T0: TryFromJavaValue<'env, 'borrow>,
        Res: ReturnValue<'env, Ret>,
        Ret: TryIntoJavaValue<'env>,
        <Ret as TryIntoJavaValue<'env>>::Target: From<jobject>,
{}


#[macro_export]
macro_rules! jni_static_function {
    ($name:expr, $param:ty, $ret:ty, $java_name:expr) => {{
        unsafe extern "system" fn anon<'local>(mut env: robusta_jni::jni::JNIEnv<'local>,
                                       _class: robusta_jni::jni::objects::JClass<'local>,
                                       a: <$param as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target {
            use $crate::{JNIStaticFunction1, JNIEnvStaticFunction1};
            $name.call_handle_error(&env, a)
        }
        let anon_func = anon as unsafe extern "system" fn(robusta_jni::jni::JNIEnv<'local>,
                                       robusta_jni::jni::objects::JClass<'local>,
                                       <$param as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target;
        // TODO: impl and use func.as_ptr()
        let anon_ptr = anon_func as *mut std::ffi::c_void;

        robusta_jni::jni::NativeMethod {
            name: robusta_jni::jni::strings::JNIString::from($java_name),
            sig: robusta_jni::jni::strings::JNIString::from(format!("({}){}",
                <<$param as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source as robusta_jni::convert::Signature>::SIG_TYPE,
                <<$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target as robusta_jni::convert::Signature>::SIG_TYPE)),
            fn_ptr: anon_ptr,
        }
    }};
    ($name:expr, $($param:ty)*, $ret:ty, $java_name:expr) => {
        compile_error!("arity must be between 1..=1") // TODO: Impl more
    };
}