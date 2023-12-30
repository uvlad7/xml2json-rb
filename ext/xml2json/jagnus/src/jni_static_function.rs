use seq_macro::seq;

use std::panic::AssertUnwindSafe;
use robusta_jni::convert::{TryFromJavaValue, TryIntoJavaValue};
use robusta_jni::jni::JNIEnv;
use crate::ReturnValue;
use crate::raise;
use crate::Error;

macro_rules! jni_static_function_n {
    ($name:ident, $env_name:ident, $n:literal) => {
        seq!(N in 0..$n {
/// Helper trait for wrapping a function as a Java static method ignoring
/// self and taking N arguments, with type conversions and error
/// handling.
///
/// See the [`jni_static_function`](crate::jni_static_function!) macro.
#[doc(hidden)]
pub trait $name<'env: 'borrow, 'borrow, Func, #(T~N,)* Res, Ret>
    where
        Self: Sized + Fn(#(T~N,)*) -> Res,
        #(T~N: TryFromJavaValue<'env, 'borrow>,)*
        Res: ReturnValue<'env, Ret>,
        Ret: TryIntoJavaValue<'env>,
{
    #[inline]
    fn call_convert_value(self, #(arg~N: T~N::Source,)* env: &'borrow JNIEnv<'env>) -> Result<Ret::Target, Error> {
        #(let conv_arg~N: T~N = TryFromJavaValue::try_from(arg~N, env)?;)*
        (self)(
            #(conv_arg~N,)*
        ).into_return_value(env)
    }

    #[inline]
    unsafe fn call_handle_error(self, #(arg~N: T~N::Source,)* env: &'borrow JNIEnv<'env>) -> Ret::Target {
        let res =
            match std::panic::catch_unwind(AssertUnwindSafe(|| {
                self.call_convert_value(#(arg~N,)* env)
            })) {
                Ok(v) => v,
                Err(e) => Error::from_panic(e, env), // Err(Error::from_panic(e, env)),
            };
        match res {
            Ok(v) => v,
            Err(e) => raise(e, env),
        }
    }
}

impl<'env: 'borrow, 'borrow, Func, #(T~N,)* Res, Ret> $name<'env, 'borrow, Func, #(T~N,)* Res, Ret> for Func
    where
        Func: Fn(#(T~N,)*) -> Res,
        #(T~N: TryFromJavaValue<'env, 'borrow>,)*
        Res: ReturnValue<'env, Ret>,
        Ret: TryIntoJavaValue<'env>,
{}

/// Helper trait for wrapping a function as a Java static method
/// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
/// type conversions and error handling.
///
/// See the [`jni_static_function`](crate::jni_static_function!) macro.
#[doc(hidden)]
pub trait $env_name<'env: 'borrow, 'borrow, Func, #(T~N,)* Res, Ret>
    where
        Self: Sized + Fn(#(T~N,)* &JNIEnv<'env>) -> Res,
        #(T~N: TryFromJavaValue<'env, 'borrow>,)*
        Res: ReturnValue<'env, Ret>,
        Ret: TryIntoJavaValue<'env>,
{
    #[inline]
    fn call_convert_value(self, #(arg~N: T~N::Source,)* env: &'borrow JNIEnv<'env>) -> Result<Ret::Target, Error> {
        #(let conv_arg~N: T~N = TryFromJavaValue::try_from(arg~N, env)?;)*
        (self)(
            #(conv_arg~N,)*
            env,
        ).into_return_value(env)
    }

    #[inline]
    unsafe fn call_handle_error(self, #(arg~N: T~N::Source,)* env: &'borrow JNIEnv<'env>) -> Ret::Target {
        let res =
            match std::panic::catch_unwind(AssertUnwindSafe(|| {
                self.call_convert_value(#(arg~N,)* env)
            })) {
                Ok(v) => v,
                Err(e) => Error::from_panic(e, env), // Err(Error::from_panic(e, env)),
            };
        match res {
            Ok(v) => v,
            Err(e) => raise(e, env),
        }
    }
}

impl<'env: 'borrow, 'borrow, Func, #(T~N,)* Res, Ret> $env_name<'env, 'borrow, Func, #(T~N,)* Res, Ret> for Func
    where
        Func: Fn(#(T~N,)* &JNIEnv<'env>) -> Res,
        #(T~N: TryFromJavaValue<'env, 'borrow>,)*
        Res: ReturnValue<'env, Ret>,
        Ret: TryIntoJavaValue<'env>,
{}
        });
    }
}

seq!(N in 0..=16 {
    jni_static_function_n!(JNIStaticFunction~N, JNIEnvStaticFunction~N, N);
});

#[macro_export]
// auto_generated block
// ruby jni_static_function.rb | xclip -selection c
macro_rules! jni_static_function {
    ($name:expr, $ret:ty, $java_name:expr) => {{
        unsafe extern "system" fn anon<'local>(mut env: robusta_jni::jni::JNIEnv<'local>,
                                       _class: robusta_jni::jni::objects::JClass<'local>,

        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target {
            use $crate::{JNIStaticFunction0, JNIEnvStaticFunction0};
            $name.call_handle_error(, &env)
        }
        let anon_func = anon as unsafe extern "system" fn(robusta_jni::jni::JNIEnv<'local>,
                                       robusta_jni::jni::objects::JClass<'local>,

        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target;
        // TODO: impl and use func.as_ptr()
        let anon_ptr = anon_func as *mut std::ffi::c_void;

        robusta_jni::jni::NativeMethod {
            name: robusta_jni::jni::strings::JNIString::from($java_name),
            sig: robusta_jni::jni::strings::JNIString::from(format!("(){}",

                <$ret as robusta_jni::convert::Signature>::SIG_TYPE)),
            fn_ptr: anon_ptr,
        }
    }};
    ($name:expr, $param1:ty, $ret:ty, $java_name:expr) => {{
        unsafe extern "system" fn anon<'local>(mut env: robusta_jni::jni::JNIEnv<'local>,
                                       _class: robusta_jni::jni::objects::JClass<'local>,
                                       a: <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target {
            use $crate::{JNIStaticFunction1, JNIEnvStaticFunction1};
            $name.call_handle_error(a, &env)
        }
        let anon_func = anon as unsafe extern "system" fn(robusta_jni::jni::JNIEnv<'local>,
                                       robusta_jni::jni::objects::JClass<'local>,
                                       <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target;
        // TODO: impl and use func.as_ptr()
        let anon_ptr = anon_func as *mut std::ffi::c_void;

        robusta_jni::jni::NativeMethod {
            name: robusta_jni::jni::strings::JNIString::from($java_name),
            sig: robusta_jni::jni::strings::JNIString::from(format!("({}){}",
                <$param1 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$ret as robusta_jni::convert::Signature>::SIG_TYPE)),
            fn_ptr: anon_ptr,
        }
    }};
    ($name:expr, $param1:ty, $param2:ty, $ret:ty, $java_name:expr) => {{
        unsafe extern "system" fn anon<'local>(mut env: robusta_jni::jni::JNIEnv<'local>,
                                       _class: robusta_jni::jni::objects::JClass<'local>,
                                       a: <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       b: <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target {
            use $crate::{JNIStaticFunction2, JNIEnvStaticFunction2};
            $name.call_handle_error(a, b, &env)
        }
        let anon_func = anon as unsafe extern "system" fn(robusta_jni::jni::JNIEnv<'local>,
                                       robusta_jni::jni::objects::JClass<'local>,
                                       <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target;
        // TODO: impl and use func.as_ptr()
        let anon_ptr = anon_func as *mut std::ffi::c_void;

        robusta_jni::jni::NativeMethod {
            name: robusta_jni::jni::strings::JNIString::from($java_name),
            sig: robusta_jni::jni::strings::JNIString::from(format!("({}{}){}",
                <$param1 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param2 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$ret as robusta_jni::convert::Signature>::SIG_TYPE)),
            fn_ptr: anon_ptr,
        }
    }};
    ($name:expr, $param1:ty, $param2:ty, $param3:ty, $ret:ty, $java_name:expr) => {{
        unsafe extern "system" fn anon<'local>(mut env: robusta_jni::jni::JNIEnv<'local>,
                                       _class: robusta_jni::jni::objects::JClass<'local>,
                                       a: <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       b: <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       c: <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target {
            use $crate::{JNIStaticFunction3, JNIEnvStaticFunction3};
            $name.call_handle_error(a, b, c, &env)
        }
        let anon_func = anon as unsafe extern "system" fn(robusta_jni::jni::JNIEnv<'local>,
                                       robusta_jni::jni::objects::JClass<'local>,
                                       <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target;
        // TODO: impl and use func.as_ptr()
        let anon_ptr = anon_func as *mut std::ffi::c_void;

        robusta_jni::jni::NativeMethod {
            name: robusta_jni::jni::strings::JNIString::from($java_name),
            sig: robusta_jni::jni::strings::JNIString::from(format!("({}{}{}){}",
                <$param1 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param2 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param3 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$ret as robusta_jni::convert::Signature>::SIG_TYPE)),
            fn_ptr: anon_ptr,
        }
    }};
    ($name:expr, $param1:ty, $param2:ty, $param3:ty, $param4:ty, $ret:ty, $java_name:expr) => {{
        unsafe extern "system" fn anon<'local>(mut env: robusta_jni::jni::JNIEnv<'local>,
                                       _class: robusta_jni::jni::objects::JClass<'local>,
                                       a: <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       b: <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       c: <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       d: <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target {
            use $crate::{JNIStaticFunction4, JNIEnvStaticFunction4};
            $name.call_handle_error(a, b, c, d, &env)
        }
        let anon_func = anon as unsafe extern "system" fn(robusta_jni::jni::JNIEnv<'local>,
                                       robusta_jni::jni::objects::JClass<'local>,
                                       <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target;
        // TODO: impl and use func.as_ptr()
        let anon_ptr = anon_func as *mut std::ffi::c_void;

        robusta_jni::jni::NativeMethod {
            name: robusta_jni::jni::strings::JNIString::from($java_name),
            sig: robusta_jni::jni::strings::JNIString::from(format!("({}{}{}{}){}",
                <$param1 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param2 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param3 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param4 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$ret as robusta_jni::convert::Signature>::SIG_TYPE)),
            fn_ptr: anon_ptr,
        }
    }};
    ($name:expr, $param1:ty, $param2:ty, $param3:ty, $param4:ty, $param5:ty, $ret:ty, $java_name:expr) => {{
        unsafe extern "system" fn anon<'local>(mut env: robusta_jni::jni::JNIEnv<'local>,
                                       _class: robusta_jni::jni::objects::JClass<'local>,
                                       a: <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       b: <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       c: <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       d: <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       e: <$param5 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target {
            use $crate::{JNIStaticFunction5, JNIEnvStaticFunction5};
            $name.call_handle_error(a, b, c, d, e, &env)
        }
        let anon_func = anon as unsafe extern "system" fn(robusta_jni::jni::JNIEnv<'local>,
                                       robusta_jni::jni::objects::JClass<'local>,
                                       <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param5 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target;
        // TODO: impl and use func.as_ptr()
        let anon_ptr = anon_func as *mut std::ffi::c_void;

        robusta_jni::jni::NativeMethod {
            name: robusta_jni::jni::strings::JNIString::from($java_name),
            sig: robusta_jni::jni::strings::JNIString::from(format!("({}{}{}{}{}){}",
                <$param1 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param2 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param3 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param4 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param5 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$ret as robusta_jni::convert::Signature>::SIG_TYPE)),
            fn_ptr: anon_ptr,
        }
    }};
    ($name:expr, $param1:ty, $param2:ty, $param3:ty, $param4:ty, $param5:ty, $param6:ty, $ret:ty, $java_name:expr) => {{
        unsafe extern "system" fn anon<'local>(mut env: robusta_jni::jni::JNIEnv<'local>,
                                       _class: robusta_jni::jni::objects::JClass<'local>,
                                       a: <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       b: <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       c: <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       d: <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       e: <$param5 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       f: <$param6 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target {
            use $crate::{JNIStaticFunction6, JNIEnvStaticFunction6};
            $name.call_handle_error(a, b, c, d, e, f, &env)
        }
        let anon_func = anon as unsafe extern "system" fn(robusta_jni::jni::JNIEnv<'local>,
                                       robusta_jni::jni::objects::JClass<'local>,
                                       <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param5 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param6 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target;
        // TODO: impl and use func.as_ptr()
        let anon_ptr = anon_func as *mut std::ffi::c_void;

        robusta_jni::jni::NativeMethod {
            name: robusta_jni::jni::strings::JNIString::from($java_name),
            sig: robusta_jni::jni::strings::JNIString::from(format!("({}{}{}{}{}{}){}",
                <$param1 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param2 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param3 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param4 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param5 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param6 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$ret as robusta_jni::convert::Signature>::SIG_TYPE)),
            fn_ptr: anon_ptr,
        }
    }};
    ($name:expr, $param1:ty, $param2:ty, $param3:ty, $param4:ty, $param5:ty, $param6:ty, $param7:ty, $ret:ty, $java_name:expr) => {{
        unsafe extern "system" fn anon<'local>(mut env: robusta_jni::jni::JNIEnv<'local>,
                                       _class: robusta_jni::jni::objects::JClass<'local>,
                                       a: <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       b: <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       c: <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       d: <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       e: <$param5 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       f: <$param6 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       g: <$param7 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target {
            use $crate::{JNIStaticFunction7, JNIEnvStaticFunction7};
            $name.call_handle_error(a, b, c, d, e, f, g, &env)
        }
        let anon_func = anon as unsafe extern "system" fn(robusta_jni::jni::JNIEnv<'local>,
                                       robusta_jni::jni::objects::JClass<'local>,
                                       <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param5 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param6 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param7 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target;
        // TODO: impl and use func.as_ptr()
        let anon_ptr = anon_func as *mut std::ffi::c_void;

        robusta_jni::jni::NativeMethod {
            name: robusta_jni::jni::strings::JNIString::from($java_name),
            sig: robusta_jni::jni::strings::JNIString::from(format!("({}{}{}{}{}{}{}){}",
                <$param1 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param2 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param3 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param4 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param5 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param6 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param7 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$ret as robusta_jni::convert::Signature>::SIG_TYPE)),
            fn_ptr: anon_ptr,
        }
    }};
    ($name:expr, $param1:ty, $param2:ty, $param3:ty, $param4:ty, $param5:ty, $param6:ty, $param7:ty, $param8:ty, $ret:ty, $java_name:expr) => {{
        unsafe extern "system" fn anon<'local>(mut env: robusta_jni::jni::JNIEnv<'local>,
                                       _class: robusta_jni::jni::objects::JClass<'local>,
                                       a: <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       b: <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       c: <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       d: <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       e: <$param5 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       f: <$param6 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       g: <$param7 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       h: <$param8 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target {
            use $crate::{JNIStaticFunction8, JNIEnvStaticFunction8};
            $name.call_handle_error(a, b, c, d, e, f, g, h, &env)
        }
        let anon_func = anon as unsafe extern "system" fn(robusta_jni::jni::JNIEnv<'local>,
                                       robusta_jni::jni::objects::JClass<'local>,
                                       <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param5 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param6 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param7 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param8 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target;
        // TODO: impl and use func.as_ptr()
        let anon_ptr = anon_func as *mut std::ffi::c_void;

        robusta_jni::jni::NativeMethod {
            name: robusta_jni::jni::strings::JNIString::from($java_name),
            sig: robusta_jni::jni::strings::JNIString::from(format!("({}{}{}{}{}{}{}{}){}",
                <$param1 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param2 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param3 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param4 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param5 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param6 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param7 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param8 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$ret as robusta_jni::convert::Signature>::SIG_TYPE)),
            fn_ptr: anon_ptr,
        }
    }};
    ($name:expr, $param1:ty, $param2:ty, $param3:ty, $param4:ty, $param5:ty, $param6:ty, $param7:ty, $param8:ty, $param9:ty, $ret:ty, $java_name:expr) => {{
        unsafe extern "system" fn anon<'local>(mut env: robusta_jni::jni::JNIEnv<'local>,
                                       _class: robusta_jni::jni::objects::JClass<'local>,
                                       a: <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       b: <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       c: <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       d: <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       e: <$param5 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       f: <$param6 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       g: <$param7 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       h: <$param8 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       i: <$param9 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target {
            use $crate::{JNIStaticFunction9, JNIEnvStaticFunction9};
            $name.call_handle_error(a, b, c, d, e, f, g, h, i, &env)
        }
        let anon_func = anon as unsafe extern "system" fn(robusta_jni::jni::JNIEnv<'local>,
                                       robusta_jni::jni::objects::JClass<'local>,
                                       <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param5 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param6 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param7 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param8 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param9 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target;
        // TODO: impl and use func.as_ptr()
        let anon_ptr = anon_func as *mut std::ffi::c_void;

        robusta_jni::jni::NativeMethod {
            name: robusta_jni::jni::strings::JNIString::from($java_name),
            sig: robusta_jni::jni::strings::JNIString::from(format!("({}{}{}{}{}{}{}{}{}){}",
                <$param1 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param2 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param3 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param4 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param5 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param6 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param7 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param8 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param9 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$ret as robusta_jni::convert::Signature>::SIG_TYPE)),
            fn_ptr: anon_ptr,
        }
    }};
    ($name:expr, $param1:ty, $param2:ty, $param3:ty, $param4:ty, $param5:ty, $param6:ty, $param7:ty, $param8:ty, $param9:ty, $param10:ty, $ret:ty, $java_name:expr) => {{
        unsafe extern "system" fn anon<'local>(mut env: robusta_jni::jni::JNIEnv<'local>,
                                       _class: robusta_jni::jni::objects::JClass<'local>,
                                       a: <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       b: <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       c: <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       d: <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       e: <$param5 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       f: <$param6 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       g: <$param7 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       h: <$param8 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       i: <$param9 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       j: <$param10 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target {
            use $crate::{JNIStaticFunction10, JNIEnvStaticFunction10};
            $name.call_handle_error(a, b, c, d, e, f, g, h, i, j, &env)
        }
        let anon_func = anon as unsafe extern "system" fn(robusta_jni::jni::JNIEnv<'local>,
                                       robusta_jni::jni::objects::JClass<'local>,
                                       <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param5 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param6 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param7 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param8 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param9 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param10 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target;
        // TODO: impl and use func.as_ptr()
        let anon_ptr = anon_func as *mut std::ffi::c_void;

        robusta_jni::jni::NativeMethod {
            name: robusta_jni::jni::strings::JNIString::from($java_name),
            sig: robusta_jni::jni::strings::JNIString::from(format!("({}{}{}{}{}{}{}{}{}{}){}",
                <$param1 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param2 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param3 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param4 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param5 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param6 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param7 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param8 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param9 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param10 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$ret as robusta_jni::convert::Signature>::SIG_TYPE)),
            fn_ptr: anon_ptr,
        }
    }};
    ($name:expr, $param1:ty, $param2:ty, $param3:ty, $param4:ty, $param5:ty, $param6:ty, $param7:ty, $param8:ty, $param9:ty, $param10:ty, $param11:ty, $ret:ty, $java_name:expr) => {{
        unsafe extern "system" fn anon<'local>(mut env: robusta_jni::jni::JNIEnv<'local>,
                                       _class: robusta_jni::jni::objects::JClass<'local>,
                                       a: <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       b: <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       c: <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       d: <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       e: <$param5 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       f: <$param6 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       g: <$param7 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       h: <$param8 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       i: <$param9 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       j: <$param10 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       k: <$param11 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target {
            use $crate::{JNIStaticFunction11, JNIEnvStaticFunction11};
            $name.call_handle_error(a, b, c, d, e, f, g, h, i, j, k, &env)
        }
        let anon_func = anon as unsafe extern "system" fn(robusta_jni::jni::JNIEnv<'local>,
                                       robusta_jni::jni::objects::JClass<'local>,
                                       <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param5 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param6 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param7 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param8 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param9 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param10 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param11 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target;
        // TODO: impl and use func.as_ptr()
        let anon_ptr = anon_func as *mut std::ffi::c_void;

        robusta_jni::jni::NativeMethod {
            name: robusta_jni::jni::strings::JNIString::from($java_name),
            sig: robusta_jni::jni::strings::JNIString::from(format!("({}{}{}{}{}{}{}{}{}{}{}){}",
                <$param1 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param2 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param3 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param4 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param5 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param6 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param7 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param8 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param9 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param10 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param11 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$ret as robusta_jni::convert::Signature>::SIG_TYPE)),
            fn_ptr: anon_ptr,
        }
    }};
    ($name:expr, $param1:ty, $param2:ty, $param3:ty, $param4:ty, $param5:ty, $param6:ty, $param7:ty, $param8:ty, $param9:ty, $param10:ty, $param11:ty, $param12:ty, $ret:ty, $java_name:expr) => {{
        unsafe extern "system" fn anon<'local>(mut env: robusta_jni::jni::JNIEnv<'local>,
                                       _class: robusta_jni::jni::objects::JClass<'local>,
                                       a: <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       b: <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       c: <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       d: <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       e: <$param5 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       f: <$param6 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       g: <$param7 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       h: <$param8 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       i: <$param9 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       j: <$param10 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       k: <$param11 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       l: <$param12 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target {
            use $crate::{JNIStaticFunction12, JNIEnvStaticFunction12};
            $name.call_handle_error(a, b, c, d, e, f, g, h, i, j, k, l, &env)
        }
        let anon_func = anon as unsafe extern "system" fn(robusta_jni::jni::JNIEnv<'local>,
                                       robusta_jni::jni::objects::JClass<'local>,
                                       <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param5 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param6 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param7 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param8 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param9 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param10 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param11 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param12 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target;
        // TODO: impl and use func.as_ptr()
        let anon_ptr = anon_func as *mut std::ffi::c_void;

        robusta_jni::jni::NativeMethod {
            name: robusta_jni::jni::strings::JNIString::from($java_name),
            sig: robusta_jni::jni::strings::JNIString::from(format!("({}{}{}{}{}{}{}{}{}{}{}{}){}",
                <$param1 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param2 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param3 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param4 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param5 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param6 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param7 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param8 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param9 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param10 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param11 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param12 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$ret as robusta_jni::convert::Signature>::SIG_TYPE)),
            fn_ptr: anon_ptr,
        }
    }};
    ($name:expr, $param1:ty, $param2:ty, $param3:ty, $param4:ty, $param5:ty, $param6:ty, $param7:ty, $param8:ty, $param9:ty, $param10:ty, $param11:ty, $param12:ty, $param13:ty, $ret:ty, $java_name:expr) => {{
        unsafe extern "system" fn anon<'local>(mut env: robusta_jni::jni::JNIEnv<'local>,
                                       _class: robusta_jni::jni::objects::JClass<'local>,
                                       a: <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       b: <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       c: <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       d: <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       e: <$param5 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       f: <$param6 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       g: <$param7 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       h: <$param8 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       i: <$param9 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       j: <$param10 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       k: <$param11 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       l: <$param12 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       m: <$param13 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target {
            use $crate::{JNIStaticFunction13, JNIEnvStaticFunction13};
            $name.call_handle_error(a, b, c, d, e, f, g, h, i, j, k, l, m, &env)
        }
        let anon_func = anon as unsafe extern "system" fn(robusta_jni::jni::JNIEnv<'local>,
                                       robusta_jni::jni::objects::JClass<'local>,
                                       <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param5 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param6 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param7 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param8 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param9 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param10 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param11 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param12 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param13 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target;
        // TODO: impl and use func.as_ptr()
        let anon_ptr = anon_func as *mut std::ffi::c_void;

        robusta_jni::jni::NativeMethod {
            name: robusta_jni::jni::strings::JNIString::from($java_name),
            sig: robusta_jni::jni::strings::JNIString::from(format!("({}{}{}{}{}{}{}{}{}{}{}{}{}){}",
                <$param1 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param2 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param3 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param4 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param5 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param6 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param7 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param8 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param9 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param10 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param11 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param12 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param13 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$ret as robusta_jni::convert::Signature>::SIG_TYPE)),
            fn_ptr: anon_ptr,
        }
    }};
    ($name:expr, $param1:ty, $param2:ty, $param3:ty, $param4:ty, $param5:ty, $param6:ty, $param7:ty, $param8:ty, $param9:ty, $param10:ty, $param11:ty, $param12:ty, $param13:ty, $param14:ty, $ret:ty, $java_name:expr) => {{
        unsafe extern "system" fn anon<'local>(mut env: robusta_jni::jni::JNIEnv<'local>,
                                       _class: robusta_jni::jni::objects::JClass<'local>,
                                       a: <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       b: <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       c: <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       d: <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       e: <$param5 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       f: <$param6 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       g: <$param7 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       h: <$param8 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       i: <$param9 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       j: <$param10 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       k: <$param11 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       l: <$param12 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       m: <$param13 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       n: <$param14 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target {
            use $crate::{JNIStaticFunction14, JNIEnvStaticFunction14};
            $name.call_handle_error(a, b, c, d, e, f, g, h, i, j, k, l, m, n, &env)
        }
        let anon_func = anon as unsafe extern "system" fn(robusta_jni::jni::JNIEnv<'local>,
                                       robusta_jni::jni::objects::JClass<'local>,
                                       <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param5 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param6 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param7 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param8 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param9 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param10 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param11 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param12 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param13 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param14 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target;
        // TODO: impl and use func.as_ptr()
        let anon_ptr = anon_func as *mut std::ffi::c_void;

        robusta_jni::jni::NativeMethod {
            name: robusta_jni::jni::strings::JNIString::from($java_name),
            sig: robusta_jni::jni::strings::JNIString::from(format!("({}{}{}{}{}{}{}{}{}{}{}{}{}{}){}",
                <$param1 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param2 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param3 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param4 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param5 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param6 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param7 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param8 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param9 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param10 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param11 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param12 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param13 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param14 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$ret as robusta_jni::convert::Signature>::SIG_TYPE)),
            fn_ptr: anon_ptr,
        }
    }};
    ($name:expr, $param1:ty, $param2:ty, $param3:ty, $param4:ty, $param5:ty, $param6:ty, $param7:ty, $param8:ty, $param9:ty, $param10:ty, $param11:ty, $param12:ty, $param13:ty, $param14:ty, $param15:ty, $ret:ty, $java_name:expr) => {{
        unsafe extern "system" fn anon<'local>(mut env: robusta_jni::jni::JNIEnv<'local>,
                                       _class: robusta_jni::jni::objects::JClass<'local>,
                                       a: <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       b: <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       c: <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       d: <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       e: <$param5 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       f: <$param6 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       g: <$param7 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       h: <$param8 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       i: <$param9 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       j: <$param10 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       k: <$param11 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       l: <$param12 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       m: <$param13 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       n: <$param14 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       o: <$param15 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target {
            use $crate::{JNIStaticFunction15, JNIEnvStaticFunction15};
            $name.call_handle_error(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, &env)
        }
        let anon_func = anon as unsafe extern "system" fn(robusta_jni::jni::JNIEnv<'local>,
                                       robusta_jni::jni::objects::JClass<'local>,
                                       <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param5 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param6 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param7 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param8 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param9 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param10 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param11 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param12 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param13 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param14 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param15 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target;
        // TODO: impl and use func.as_ptr()
        let anon_ptr = anon_func as *mut std::ffi::c_void;

        robusta_jni::jni::NativeMethod {
            name: robusta_jni::jni::strings::JNIString::from($java_name),
            sig: robusta_jni::jni::strings::JNIString::from(format!("({}{}{}{}{}{}{}{}{}{}{}{}{}{}{}){}",
                <$param1 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param2 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param3 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param4 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param5 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param6 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param7 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param8 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param9 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param10 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param11 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param12 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param13 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param14 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param15 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$ret as robusta_jni::convert::Signature>::SIG_TYPE)),
            fn_ptr: anon_ptr,
        }
    }};
    ($name:expr, $param1:ty, $param2:ty, $param3:ty, $param4:ty, $param5:ty, $param6:ty, $param7:ty, $param8:ty, $param9:ty, $param10:ty, $param11:ty, $param12:ty, $param13:ty, $param14:ty, $param15:ty, $param16:ty, $ret:ty, $java_name:expr) => {{
        unsafe extern "system" fn anon<'local>(mut env: robusta_jni::jni::JNIEnv<'local>,
                                       _class: robusta_jni::jni::objects::JClass<'local>,
                                       a: <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       b: <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       c: <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       d: <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       e: <$param5 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       f: <$param6 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       g: <$param7 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       h: <$param8 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       i: <$param9 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       j: <$param10 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       k: <$param11 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       l: <$param12 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       m: <$param13 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       n: <$param14 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       o: <$param15 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       p: <$param16 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target {
            use $crate::{JNIStaticFunction16, JNIEnvStaticFunction16};
            $name.call_handle_error(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, &env)
        }
        let anon_func = anon as unsafe extern "system" fn(robusta_jni::jni::JNIEnv<'local>,
                                       robusta_jni::jni::objects::JClass<'local>,
                                       <$param1 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param2 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param3 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param4 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param5 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param6 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param7 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param8 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param9 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param10 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param11 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param12 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param13 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param14 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param15 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
                                       <$param16 as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source,
        ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target;
        // TODO: impl and use func.as_ptr()
        let anon_ptr = anon_func as *mut std::ffi::c_void;

        robusta_jni::jni::NativeMethod {
            name: robusta_jni::jni::strings::JNIString::from($java_name),
            sig: robusta_jni::jni::strings::JNIString::from(format!("({}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}){}",
                <$param1 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param2 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param3 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param4 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param5 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param6 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param7 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param8 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param9 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param10 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param11 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param12 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param13 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param14 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param15 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$param16 as robusta_jni::convert::Signature>::SIG_TYPE,
                <$ret as robusta_jni::convert::Signature>::SIG_TYPE)),
            fn_ptr: anon_ptr,
        }
    }};
    ($name:expr, $($param:ty)*, $ret:ty, $java_name:expr) => {
        compile_error!("arity must be between 0..=16")
    };
}
// end of auto_generated block
