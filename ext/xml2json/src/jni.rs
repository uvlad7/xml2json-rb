// pub mod errors {
//     use std::borrow::Cow;
//
//     #[derive(Debug)]
//     pub enum Error {
//         /// An error generated in Rust code that will raise an exception when
//         /// returned to Java.
//         /// Todo: change first arg type to "Throwable"
//         Error(Cow<'static, str>, Cow<'static, str>),
//     }
//
//     impl Error {
//         pub fn new<F, T>(class: F, msg: T) -> Self
//             where
//                 F: Into<Cow<'static, str>>,
//                 T: Into<Cow<'static, str>>,
//         {
//             Self::Error(class.into(), msg.into())
//         }
//     }
//
//     pub fn type_error(msg: String) -> Error {
//         Error::new("java/lang/RuntimeException", msg)
//     }
//
//     pub fn runtime_error(msg: String) -> Error {
//         Error::new("java/lang/ClassCastException", msg)
//     }
// }
//
// pub mod args {
//     use jni::JNIEnv;
//     use jni::objects::{JObject, JString};
//     use crate::implementation::errors::{Error, runtime_error};
//
//     type JHash<'local> = JObject<'local>;
//
//     pub struct Args<'local> {
//         pub str_arg: String,
//         opts: Option<JObject<'local>>,
//     }
//
//     pub struct Opts<'local> {
//         opts_hash: JHash<'local>,
//     }
//
//     impl Args<'_> {
//         pub fn new<'local>(mut env: JNIEnv<'local>, str: JString<'local>, opts: JObject<'local>) -> Result<Args<'local>, Error> {
//             let j_res = env.get_string(&str);
//             match j_res {
//                 Ok(java_str) => { Ok(Args { str_arg: java_str.into(), opts: Some(opts) }) }
//                 Err(err) => { Err(runtime_error("".to_string())) }
//             }
//         }
//
//         pub fn opts<'local>(&self) -> Option<Opts<'local>> {
//             self.opts.map(|opts_hash| Opts { opts_hash })
//         }
//     }
//
//     impl Opts<'_> {
//         pub fn lookup<U>(&self, _key: &str) -> Result<Option<U>, Error> {
//             Err(runtime_error("Not implemented".to_string()))
//         }
//
//         pub fn lookup_str(&self, _key: &str) -> Result<Option<String>, Error> {
//             Err(runtime_error("Not implemented".to_string()))
//         }
//     }
// }

use std::any::Any;
use std::borrow::Cow;
use std::marker::PhantomData;
use std::panic::AssertUnwindSafe;
use robusta_jni::convert::{TryFromJavaValue, TryIntoJavaValue};
use robusta_jni::jni::JNIEnv;
use robusta_jni::jni::sys::jobject;

pub enum Error {
    /// An error generated in Rust code that will raise an exception when
    /// returned to Java.
    Error(&'static str, Cow<'static, str>),
    /// A Java Exception captured from Java as an Error.
    Exception(robusta_jni::jni::errors::Error),
}

impl From<robusta_jni::jni::errors::Error> for Error {
    fn from(val: robusta_jni::jni::errors::Error) -> Self {
        Self::Exception(val)
    }
}

impl Error {
    fn from_panic<'local>(e: Box<dyn Any + Send + 'static>, env: &JNIEnv<'local>) -> ! {
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
    match e {
        Error::Exception(jni_err) => {
            // TODO: Impl ToException trait
            let _ = env.throw_new("java/lang/RuntimeException", format!("{:?}", jni_err));
        }
        Error::Error(cls, msg) => {
            let _ = env.throw_new(cls, msg);
        }
    };
    T::from(std::ptr::null_mut())
}

/// Helper type for wrapping a function as a Java static method and
/// taking 1 argument, with type conversions and error handling.
///
/// See the [`function`](crate::jni_function!) macro.
#[doc(hidden)]
pub struct Function1<Func, A, Res, Ret> {
    func: Func,
    a: PhantomData<A>,
    res: PhantomData<Res>,
    ret: PhantomData<Ret>,
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

#[allow(missing_docs)]
impl<'env: 'borrow, 'borrow, Func, A, Res, Ret> Function1<Func, A, Res, Ret>
// 'env - same as 'local
    where
        Func: Fn(A) -> Res,
        A: TryFromJavaValue<'env, 'borrow>,
        Res: ReturnValue<'env, Ret>,
        Ret: TryIntoJavaValue<'env>,
        <Ret as TryIntoJavaValue<'env>>::Target: From<jobject>,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            a: Default::default(),
            res: Default::default(),
            ret: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(self, a: A::Source, env: &'borrow JNIEnv<'env>) -> Result<Ret::Target, Error> {
        let arg: A = TryFromJavaValue::try_from(a, env)?;
        let res = (self.func)(arg);
        res.into_return_value(env)
    }

    #[inline]
    pub unsafe fn call_handle_error(self, a: A::Source, env: &'borrow JNIEnv<'env>) -> Ret::Target {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| self.call_convert_value(a, env))) {
            Ok(v) => v,
            Err(e) => Error::from_panic(e, env), // Err(Error::from_panic(e, env)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e, env)
        }
    }
}

#[macro_export]
macro_rules! jni_function {
    ($name:expr, $param:ty, $ret:ty, $java_name:expr) => {{
        unsafe extern "system" fn anon<'local>(mut env: JNIEnv<'local>,
                                       _class: JClass<'local>,
                                       a: <$param as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target {
            $crate::jni::Function1::new($name).call_handle_error(a, &env)
        }
        let anon_func = anon as unsafe extern "system" fn(env: JNIEnv<'local>, _class: JClass<'local>, a: <$param as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target;
        // like func.as_ptr()
        let anon_ptr = anon_func as *mut c_void;

        NativeMethod {
            name: JNIString::from($java_name),
            sig: JNIString::from(format!("({}){}",
                <<$param as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source as robusta_jni::convert::Signature>::SIG_TYPE,
                <<$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target as robusta_jni::convert::Signature>::SIG_TYPE)),
            fn_ptr: anon_ptr,
        }
    }};
    ($name:expr, $($param:ty)*, $ret:ty, $java_name:expr) => {
        compile_error!("arity must be between 1..=1") // TODO: Impl more
    };
}
