use std::any::Any;
use std::borrow::Cow;

use robusta_jni::convert::TryIntoJavaValue;
use robusta_jni::jni::JNIEnv;

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

/// # Safety
/// this is only safe to return to Java immediately
/// when an exception is thrown,
/// since the return value isn't used in that case
unsafe fn raise<T>(e: Error, env: &JNIEnv) -> T {
    // TODO: Shouldn't we clean already thrown exceptions with env.exception_clear?
    // Note: As an optimisation, it's required only in JNIEnv*Method/Function
    match e.0 {
        ErrorType::Exception(jni_err) => {
            // TODO: Impl ToException trait
            let _ = env.throw_new("java/lang/RuntimeException", format!("{:?}", jni_err));
        }
        ErrorType::Error(cls, msg) => {
            let _ = env.throw_new(cls, msg);
        }
    };
    unsafe { std::mem::MaybeUninit::uninit().assume_init() }
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

impl<'env, T> ReturnValue<'env, T> for T
    where
        T: TryIntoJavaValue<'env>,
{
    fn into_return_value(self, env: &JNIEnv<'env>) -> Result<<T as TryIntoJavaValue<'env>>::Target, Error> {
        Ok(TryIntoJavaValue::try_into(self, env)?)
    }
}

mod jni_static_function;

pub use jni_static_function::*;

mod jni_function;

pub use jni_function::*;
