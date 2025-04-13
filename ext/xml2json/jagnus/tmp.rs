#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::any::Any;
use std::borrow::Cow;
use robusta_jni::convert::TryIntoJavaValue;
use robusta_jni::jni::JNIEnv;
pub use robusta_jni;
mod java {
    mod jni_static_function {
        use seq_macro::seq;
        use std::panic::AssertUnwindSafe;
        use robusta_jni::convert::{TryFromJavaValue, TryIntoJavaValue};
        use robusta_jni::jni::JNIEnv;
        use crate::ReturnValue;
        use crate::raise;
        use crate::Error;
        /// Helper trait for wrapping a function as a Java static method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIStaticFunction0<'env: 'borrow, 'borrow, Func, Res, Ret>
        where
            Self: Sized + Fn() -> Res,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                (self)().into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| { self.call_convert_value(env) }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            Res,
            Ret,
        > JNIStaticFunction0<'env, 'borrow, Func, Res, Ret> for Func
        where
            Func: Fn() -> Res,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvStaticFunction0<'env: 'borrow, 'borrow, Func, Res, Ret>
        where
            Self: Sized + Fn(&JNIEnv<'env>) -> Res,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                (self)(env).into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| { self.call_convert_value(env) }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            Res,
            Ret,
        > JNIEnvStaticFunction0<'env, 'borrow, Func, Res, Ret> for Func
        where
            Func: Fn(&JNIEnv<'env>) -> Res,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
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
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                (self)(conv_arg0).into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| { self.call_convert_value(arg0, env) }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            Res,
            Ret,
        > JNIStaticFunction1<'env, 'borrow, Func, T0, Res, Ret> for Func
        where
            Func: Fn(T0) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvStaticFunction1<'env: 'borrow, 'borrow, Func, T0, Res, Ret>
        where
            Self: Sized + Fn(T0, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                (self)(conv_arg0, env).into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| { self.call_convert_value(arg0, env) }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            Res,
            Ret,
        > JNIEnvStaticFunction1<'env, 'borrow, Func, T0, Res, Ret> for Func
        where
            Func: Fn(T0, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIStaticFunction2<'env: 'borrow, 'borrow, Func, T0, T1, Res, Ret>
        where
            Self: Sized + Fn(T0, T1) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                (self)(conv_arg0, conv_arg1).into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| { self.call_convert_value(arg0, arg1, env) }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            Res,
            Ret,
        > JNIStaticFunction2<'env, 'borrow, Func, T0, T1, Res, Ret> for Func
        where
            Func: Fn(T0, T1) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvStaticFunction2<'env: 'borrow, 'borrow, Func, T0, T1, Res, Ret>
        where
            Self: Sized + Fn(T0, T1, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                (self)(conv_arg0, conv_arg1, env).into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| { self.call_convert_value(arg0, arg1, env) }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            Res,
            Ret,
        > JNIEnvStaticFunction2<'env, 'borrow, Func, T0, T1, Res, Ret> for Func
        where
            Func: Fn(T0, T1, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIStaticFunction3<'env: 'borrow, 'borrow, Func, T0, T1, T2, Res, Ret>
        where
            Self: Sized + Fn(T0, T1, T2) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                (self)(conv_arg0, conv_arg1, conv_arg2).into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(arg0, arg1, arg2, env)
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            Res,
            Ret,
        > JNIStaticFunction3<'env, 'borrow, Func, T0, T1, T2, Res, Ret> for Func
        where
            Func: Fn(T0, T1, T2) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvStaticFunction3<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                (self)(conv_arg0, conv_arg1, conv_arg2, env).into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(arg0, arg1, arg2, env)
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            Res,
            Ret,
        > JNIEnvStaticFunction3<'env, 'borrow, Func, T0, T1, T2, Res, Ret> for Func
        where
            Func: Fn(T0, T1, T2, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIStaticFunction4<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                (self)(conv_arg0, conv_arg1, conv_arg2, conv_arg3).into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(arg0, arg1, arg2, arg3, env)
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            Res,
            Ret,
        > JNIStaticFunction4<'env, 'borrow, Func, T0, T1, T2, T3, Res, Ret> for Func
        where
            Func: Fn(T0, T1, T2, T3) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvStaticFunction4<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                (self)(conv_arg0, conv_arg1, conv_arg2, conv_arg3, env)
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(arg0, arg1, arg2, arg3, env)
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            Res,
            Ret,
        > JNIEnvStaticFunction4<'env, 'borrow, Func, T0, T1, T2, T3, Res, Ret> for Func
        where
            Func: Fn(T0, T1, T2, T3, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIStaticFunction5<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                (self)(conv_arg0, conv_arg1, conv_arg2, conv_arg3, conv_arg4)
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(arg0, arg1, arg2, arg3, arg4, env)
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            Res,
            Ret,
        > JNIStaticFunction5<'env, 'borrow, Func, T0, T1, T2, T3, T4, Res, Ret> for Func
        where
            Func: Fn(T0, T1, T2, T3, T4) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvStaticFunction5<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                (self)(conv_arg0, conv_arg1, conv_arg2, conv_arg3, conv_arg4, env)
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(arg0, arg1, arg2, arg3, arg4, env)
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            Res,
            Ret,
        > JNIEnvStaticFunction5<'env, 'borrow, Func, T0, T1, T2, T3, T4, Res, Ret>
        for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIStaticFunction6<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4, T5) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                (self)(conv_arg0, conv_arg1, conv_arg2, conv_arg3, conv_arg4, conv_arg5)
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(arg0, arg1, arg2, arg3, arg4, arg5, env)
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            Res,
            Ret,
        > JNIStaticFunction6<'env, 'borrow, Func, T0, T1, T2, T3, T4, T5, Res, Ret>
        for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvStaticFunction6<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4, T5, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        env,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(arg0, arg1, arg2, arg3, arg4, arg5, env)
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            Res,
            Ret,
        > JNIEnvStaticFunction6<'env, 'borrow, Func, T0, T1, T2, T3, T4, T5, Res, Ret>
        for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIStaticFunction7<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4, T5, T6) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            Res,
            Ret,
        > JNIStaticFunction7<'env, 'borrow, Func, T0, T1, T2, T3, T4, T5, T6, Res, Ret>
        for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvStaticFunction7<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4, T5, T6, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        env,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            Res,
            Ret,
        > JNIEnvStaticFunction7<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIStaticFunction8<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4, T5, T6, T7) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            Res,
            Ret,
        > JNIStaticFunction8<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6, T7) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvStaticFunction8<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4, T5, T6, T7, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        env,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            Res,
            Ret,
        > JNIEnvStaticFunction8<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6, T7, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIStaticFunction9<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            Res,
            Ret,
        > JNIStaticFunction9<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvStaticFunction9<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        env,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            Res,
            Ret,
        > JNIEnvStaticFunction9<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIStaticFunction10<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            Res,
            Ret,
        > JNIStaticFunction10<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvStaticFunction10<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            Res,
            Ret,
        >
        where
            Self: Sized
                + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        env,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            Res,
            Ret,
        > JNIEnvStaticFunction10<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIStaticFunction11<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                let conv_arg10: T10 = TryFromJavaValue::try_from(arg10, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        conv_arg10,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            arg10,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            Res,
            Ret,
        > JNIStaticFunction11<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvStaticFunction11<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            Res,
            Ret,
        >
        where
            Self: Sized
                + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                let conv_arg10: T10 = TryFromJavaValue::try_from(arg10, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        conv_arg10,
                        env,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            arg10,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            Res,
            Ret,
        > JNIEnvStaticFunction11<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIStaticFunction12<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                let conv_arg10: T10 = TryFromJavaValue::try_from(arg10, env)?;
                let conv_arg11: T11 = TryFromJavaValue::try_from(arg11, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        conv_arg10,
                        conv_arg11,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            arg10,
                            arg11,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            Res,
            Ret,
        > JNIStaticFunction12<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvStaticFunction12<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            Res,
            Ret,
        >
        where
            Self: Sized
                + Fn(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    &JNIEnv<'env>,
                ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                let conv_arg10: T10 = TryFromJavaValue::try_from(arg10, env)?;
                let conv_arg11: T11 = TryFromJavaValue::try_from(arg11, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        conv_arg10,
                        conv_arg11,
                        env,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            arg10,
                            arg11,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            Res,
            Ret,
        > JNIEnvStaticFunction12<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                &JNIEnv<'env>,
            ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIStaticFunction13<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            Res,
            Ret,
        >
        where
            Self: Sized
                + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                let conv_arg10: T10 = TryFromJavaValue::try_from(arg10, env)?;
                let conv_arg11: T11 = TryFromJavaValue::try_from(arg11, env)?;
                let conv_arg12: T12 = TryFromJavaValue::try_from(arg12, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        conv_arg10,
                        conv_arg11,
                        conv_arg12,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            arg10,
                            arg11,
                            arg12,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            Res,
            Ret,
        > JNIStaticFunction13<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvStaticFunction13<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            Res,
            Ret,
        >
        where
            Self: Sized
                + Fn(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    &JNIEnv<'env>,
                ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                let conv_arg10: T10 = TryFromJavaValue::try_from(arg10, env)?;
                let conv_arg11: T11 = TryFromJavaValue::try_from(arg11, env)?;
                let conv_arg12: T12 = TryFromJavaValue::try_from(arg12, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        conv_arg10,
                        conv_arg11,
                        conv_arg12,
                        env,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            arg10,
                            arg11,
                            arg12,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            Res,
            Ret,
        > JNIEnvStaticFunction13<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                &JNIEnv<'env>,
            ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIStaticFunction14<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            Res,
            Ret,
        >
        where
            Self: Sized
                + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            T13: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                arg13: T13::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                let conv_arg10: T10 = TryFromJavaValue::try_from(arg10, env)?;
                let conv_arg11: T11 = TryFromJavaValue::try_from(arg11, env)?;
                let conv_arg12: T12 = TryFromJavaValue::try_from(arg12, env)?;
                let conv_arg13: T13 = TryFromJavaValue::try_from(arg13, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        conv_arg10,
                        conv_arg11,
                        conv_arg12,
                        conv_arg13,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                arg13: T13::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            arg10,
                            arg11,
                            arg12,
                            arg13,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            Res,
            Ret,
        > JNIStaticFunction14<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            T13: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvStaticFunction14<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            Res,
            Ret,
        >
        where
            Self: Sized
                + Fn(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    &JNIEnv<'env>,
                ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            T13: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                arg13: T13::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                let conv_arg10: T10 = TryFromJavaValue::try_from(arg10, env)?;
                let conv_arg11: T11 = TryFromJavaValue::try_from(arg11, env)?;
                let conv_arg12: T12 = TryFromJavaValue::try_from(arg12, env)?;
                let conv_arg13: T13 = TryFromJavaValue::try_from(arg13, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        conv_arg10,
                        conv_arg11,
                        conv_arg12,
                        conv_arg13,
                        env,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                arg13: T13::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            arg10,
                            arg11,
                            arg12,
                            arg13,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            Res,
            Ret,
        > JNIEnvStaticFunction14<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                &JNIEnv<'env>,
            ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            T13: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIStaticFunction15<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            Res,
            Ret,
        >
        where
            Self: Sized
                + Fn(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    T14,
                ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            T13: TryFromJavaValue<'env, 'borrow>,
            T14: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                arg13: T13::Source,
                arg14: T14::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                let conv_arg10: T10 = TryFromJavaValue::try_from(arg10, env)?;
                let conv_arg11: T11 = TryFromJavaValue::try_from(arg11, env)?;
                let conv_arg12: T12 = TryFromJavaValue::try_from(arg12, env)?;
                let conv_arg13: T13 = TryFromJavaValue::try_from(arg13, env)?;
                let conv_arg14: T14 = TryFromJavaValue::try_from(arg14, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        conv_arg10,
                        conv_arg11,
                        conv_arg12,
                        conv_arg13,
                        conv_arg14,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                arg13: T13::Source,
                arg14: T14::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            arg10,
                            arg11,
                            arg12,
                            arg13,
                            arg14,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            Res,
            Ret,
        > JNIStaticFunction15<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
            ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            T13: TryFromJavaValue<'env, 'borrow>,
            T14: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvStaticFunction15<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            Res,
            Ret,
        >
        where
            Self: Sized
                + Fn(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    T14,
                    &JNIEnv<'env>,
                ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            T13: TryFromJavaValue<'env, 'borrow>,
            T14: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                arg13: T13::Source,
                arg14: T14::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                let conv_arg10: T10 = TryFromJavaValue::try_from(arg10, env)?;
                let conv_arg11: T11 = TryFromJavaValue::try_from(arg11, env)?;
                let conv_arg12: T12 = TryFromJavaValue::try_from(arg12, env)?;
                let conv_arg13: T13 = TryFromJavaValue::try_from(arg13, env)?;
                let conv_arg14: T14 = TryFromJavaValue::try_from(arg14, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        conv_arg10,
                        conv_arg11,
                        conv_arg12,
                        conv_arg13,
                        conv_arg14,
                        env,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                arg13: T13::Source,
                arg14: T14::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            arg10,
                            arg11,
                            arg12,
                            arg13,
                            arg14,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            Res,
            Ret,
        > JNIEnvStaticFunction15<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                &JNIEnv<'env>,
            ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            T13: TryFromJavaValue<'env, 'borrow>,
            T14: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIStaticFunction16<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            T15,
            Res,
            Ret,
        >
        where
            Self: Sized
                + Fn(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    T14,
                    T15,
                ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            T13: TryFromJavaValue<'env, 'borrow>,
            T14: TryFromJavaValue<'env, 'borrow>,
            T15: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                arg13: T13::Source,
                arg14: T14::Source,
                arg15: T15::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                let conv_arg10: T10 = TryFromJavaValue::try_from(arg10, env)?;
                let conv_arg11: T11 = TryFromJavaValue::try_from(arg11, env)?;
                let conv_arg12: T12 = TryFromJavaValue::try_from(arg12, env)?;
                let conv_arg13: T13 = TryFromJavaValue::try_from(arg13, env)?;
                let conv_arg14: T14 = TryFromJavaValue::try_from(arg14, env)?;
                let conv_arg15: T15 = TryFromJavaValue::try_from(arg15, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        conv_arg10,
                        conv_arg11,
                        conv_arg12,
                        conv_arg13,
                        conv_arg14,
                        conv_arg15,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                arg13: T13::Source,
                arg14: T14::Source,
                arg15: T15::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            arg10,
                            arg11,
                            arg12,
                            arg13,
                            arg14,
                            arg15,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            T15,
            Res,
            Ret,
        > JNIStaticFunction16<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            T15,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
            ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            T13: TryFromJavaValue<'env, 'borrow>,
            T14: TryFromJavaValue<'env, 'borrow>,
            T15: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java static method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_static_function`](crate::jni_static_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvStaticFunction16<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            T15,
            Res,
            Ret,
        >
        where
            Self: Sized
                + Fn(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    T14,
                    T15,
                    &JNIEnv<'env>,
                ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            T13: TryFromJavaValue<'env, 'borrow>,
            T14: TryFromJavaValue<'env, 'borrow>,
            T15: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                arg13: T13::Source,
                arg14: T14::Source,
                arg15: T15::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                let conv_arg10: T10 = TryFromJavaValue::try_from(arg10, env)?;
                let conv_arg11: T11 = TryFromJavaValue::try_from(arg11, env)?;
                let conv_arg12: T12 = TryFromJavaValue::try_from(arg12, env)?;
                let conv_arg13: T13 = TryFromJavaValue::try_from(arg13, env)?;
                let conv_arg14: T14 = TryFromJavaValue::try_from(arg14, env)?;
                let conv_arg15: T15 = TryFromJavaValue::try_from(arg15, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        conv_arg10,
                        conv_arg11,
                        conv_arg12,
                        conv_arg13,
                        conv_arg14,
                        conv_arg15,
                        env,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                arg13: T13::Source,
                arg14: T14::Source,
                arg15: T15::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            arg10,
                            arg11,
                            arg12,
                            arg13,
                            arg14,
                            arg15,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            T15,
            Res,
            Ret,
        > JNIEnvStaticFunction16<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            T15,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                &JNIEnv<'env>,
            ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            T13: TryFromJavaValue<'env, 'borrow>,
            T14: TryFromJavaValue<'env, 'borrow>,
            T15: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
    }
    pub use jni_static_function::*;
    mod jni_function {
        use seq_macro::seq;
        use std::panic::AssertUnwindSafe;
        use robusta_jni::convert::{TryFromJavaValue, TryIntoJavaValue};
        use robusta_jni::jni::JNIEnv;
        use crate::ReturnValue;
        use crate::raise;
        use crate::Error;
        /// Helper trait for wrapping a function as a Java method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIFunction0<'env: 'borrow, 'borrow, Func, Res, Ret>
        where
            Self: Sized + Fn() -> Res,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                (self)().into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| { self.call_convert_value(env) }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            Res,
            Ret,
        > JNIFunction0<'env, 'borrow, Func, Res, Ret> for Func
        where
            Func: Fn() -> Res,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvFunction0<'env: 'borrow, 'borrow, Func, Res, Ret>
        where
            Self: Sized + Fn(&JNIEnv<'env>) -> Res,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                (self)(env).into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| { self.call_convert_value(env) }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            Res,
            Ret,
        > JNIEnvFunction0<'env, 'borrow, Func, Res, Ret> for Func
        where
            Func: Fn(&JNIEnv<'env>) -> Res,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIFunction1<'env: 'borrow, 'borrow, Func, T0, Res, Ret>
        where
            Self: Sized + Fn(T0) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                (self)(conv_arg0).into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| { self.call_convert_value(arg0, env) }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            Res,
            Ret,
        > JNIFunction1<'env, 'borrow, Func, T0, Res, Ret> for Func
        where
            Func: Fn(T0) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvFunction1<'env: 'borrow, 'borrow, Func, T0, Res, Ret>
        where
            Self: Sized + Fn(T0, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                (self)(conv_arg0, env).into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| { self.call_convert_value(arg0, env) }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            Res,
            Ret,
        > JNIEnvFunction1<'env, 'borrow, Func, T0, Res, Ret> for Func
        where
            Func: Fn(T0, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIFunction2<'env: 'borrow, 'borrow, Func, T0, T1, Res, Ret>
        where
            Self: Sized + Fn(T0, T1) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                (self)(conv_arg0, conv_arg1).into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| { self.call_convert_value(arg0, arg1, env) }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            Res,
            Ret,
        > JNIFunction2<'env, 'borrow, Func, T0, T1, Res, Ret> for Func
        where
            Func: Fn(T0, T1) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvFunction2<'env: 'borrow, 'borrow, Func, T0, T1, Res, Ret>
        where
            Self: Sized + Fn(T0, T1, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                (self)(conv_arg0, conv_arg1, env).into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| { self.call_convert_value(arg0, arg1, env) }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            Res,
            Ret,
        > JNIEnvFunction2<'env, 'borrow, Func, T0, T1, Res, Ret> for Func
        where
            Func: Fn(T0, T1, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIFunction3<'env: 'borrow, 'borrow, Func, T0, T1, T2, Res, Ret>
        where
            Self: Sized + Fn(T0, T1, T2) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                (self)(conv_arg0, conv_arg1, conv_arg2).into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(arg0, arg1, arg2, env)
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            Res,
            Ret,
        > JNIFunction3<'env, 'borrow, Func, T0, T1, T2, Res, Ret> for Func
        where
            Func: Fn(T0, T1, T2) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvFunction3<'env: 'borrow, 'borrow, Func, T0, T1, T2, Res, Ret>
        where
            Self: Sized + Fn(T0, T1, T2, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                (self)(conv_arg0, conv_arg1, conv_arg2, env).into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(arg0, arg1, arg2, env)
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            Res,
            Ret,
        > JNIEnvFunction3<'env, 'borrow, Func, T0, T1, T2, Res, Ret> for Func
        where
            Func: Fn(T0, T1, T2, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIFunction4<'env: 'borrow, 'borrow, Func, T0, T1, T2, T3, Res, Ret>
        where
            Self: Sized + Fn(T0, T1, T2, T3) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                (self)(conv_arg0, conv_arg1, conv_arg2, conv_arg3).into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(arg0, arg1, arg2, arg3, env)
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            Res,
            Ret,
        > JNIFunction4<'env, 'borrow, Func, T0, T1, T2, T3, Res, Ret> for Func
        where
            Func: Fn(T0, T1, T2, T3) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvFunction4<'env: 'borrow, 'borrow, Func, T0, T1, T2, T3, Res, Ret>
        where
            Self: Sized + Fn(T0, T1, T2, T3, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                (self)(conv_arg0, conv_arg1, conv_arg2, conv_arg3, env)
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(arg0, arg1, arg2, arg3, env)
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            Res,
            Ret,
        > JNIEnvFunction4<'env, 'borrow, Func, T0, T1, T2, T3, Res, Ret> for Func
        where
            Func: Fn(T0, T1, T2, T3, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIFunction5<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                (self)(conv_arg0, conv_arg1, conv_arg2, conv_arg3, conv_arg4)
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(arg0, arg1, arg2, arg3, arg4, env)
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            Res,
            Ret,
        > JNIFunction5<'env, 'borrow, Func, T0, T1, T2, T3, T4, Res, Ret> for Func
        where
            Func: Fn(T0, T1, T2, T3, T4) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvFunction5<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                (self)(conv_arg0, conv_arg1, conv_arg2, conv_arg3, conv_arg4, env)
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(arg0, arg1, arg2, arg3, arg4, env)
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            Res,
            Ret,
        > JNIEnvFunction5<'env, 'borrow, Func, T0, T1, T2, T3, T4, Res, Ret> for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIFunction6<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4, T5) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                (self)(conv_arg0, conv_arg1, conv_arg2, conv_arg3, conv_arg4, conv_arg5)
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(arg0, arg1, arg2, arg3, arg4, arg5, env)
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            Res,
            Ret,
        > JNIFunction6<'env, 'borrow, Func, T0, T1, T2, T3, T4, T5, Res, Ret> for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvFunction6<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4, T5, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        env,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(arg0, arg1, arg2, arg3, arg4, arg5, env)
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            Res,
            Ret,
        > JNIEnvFunction6<'env, 'borrow, Func, T0, T1, T2, T3, T4, T5, Res, Ret> for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIFunction7<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4, T5, T6) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            Res,
            Ret,
        > JNIFunction7<'env, 'borrow, Func, T0, T1, T2, T3, T4, T5, T6, Res, Ret>
        for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvFunction7<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4, T5, T6, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        env,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            Res,
            Ret,
        > JNIEnvFunction7<'env, 'borrow, Func, T0, T1, T2, T3, T4, T5, T6, Res, Ret>
        for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIFunction8<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4, T5, T6, T7) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            Res,
            Ret,
        > JNIFunction8<'env, 'borrow, Func, T0, T1, T2, T3, T4, T5, T6, T7, Res, Ret>
        for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6, T7) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvFunction8<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4, T5, T6, T7, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        env,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            Res,
            Ret,
        > JNIEnvFunction8<'env, 'borrow, Func, T0, T1, T2, T3, T4, T5, T6, T7, Res, Ret>
        for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6, T7, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIFunction9<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            Res,
            Ret,
        > JNIFunction9<'env, 'borrow, Func, T0, T1, T2, T3, T4, T5, T6, T7, T8, Res, Ret>
        for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvFunction9<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        env,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            Res,
            Ret,
        > JNIEnvFunction9<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIFunction10<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            Res,
            Ret,
        > JNIFunction10<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvFunction10<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            Res,
            Ret,
        >
        where
            Self: Sized
                + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        env,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            Res,
            Ret,
        > JNIEnvFunction10<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIFunction11<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                let conv_arg10: T10 = TryFromJavaValue::try_from(arg10, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        conv_arg10,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            arg10,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            Res,
            Ret,
        > JNIFunction11<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvFunction11<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            Res,
            Ret,
        >
        where
            Self: Sized
                + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                let conv_arg10: T10 = TryFromJavaValue::try_from(arg10, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        conv_arg10,
                        env,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            arg10,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            Res,
            Ret,
        > JNIEnvFunction11<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, &JNIEnv<'env>) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIFunction12<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            Res,
            Ret,
        >
        where
            Self: Sized + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                let conv_arg10: T10 = TryFromJavaValue::try_from(arg10, env)?;
                let conv_arg11: T11 = TryFromJavaValue::try_from(arg11, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        conv_arg10,
                        conv_arg11,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            arg10,
                            arg11,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            Res,
            Ret,
        > JNIFunction12<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvFunction12<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            Res,
            Ret,
        >
        where
            Self: Sized
                + Fn(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    &JNIEnv<'env>,
                ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                let conv_arg10: T10 = TryFromJavaValue::try_from(arg10, env)?;
                let conv_arg11: T11 = TryFromJavaValue::try_from(arg11, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        conv_arg10,
                        conv_arg11,
                        env,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            arg10,
                            arg11,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            Res,
            Ret,
        > JNIEnvFunction12<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                &JNIEnv<'env>,
            ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIFunction13<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            Res,
            Ret,
        >
        where
            Self: Sized
                + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                let conv_arg10: T10 = TryFromJavaValue::try_from(arg10, env)?;
                let conv_arg11: T11 = TryFromJavaValue::try_from(arg11, env)?;
                let conv_arg12: T12 = TryFromJavaValue::try_from(arg12, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        conv_arg10,
                        conv_arg11,
                        conv_arg12,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            arg10,
                            arg11,
                            arg12,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            Res,
            Ret,
        > JNIFunction13<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvFunction13<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            Res,
            Ret,
        >
        where
            Self: Sized
                + Fn(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    &JNIEnv<'env>,
                ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                let conv_arg10: T10 = TryFromJavaValue::try_from(arg10, env)?;
                let conv_arg11: T11 = TryFromJavaValue::try_from(arg11, env)?;
                let conv_arg12: T12 = TryFromJavaValue::try_from(arg12, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        conv_arg10,
                        conv_arg11,
                        conv_arg12,
                        env,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            arg10,
                            arg11,
                            arg12,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            Res,
            Ret,
        > JNIEnvFunction13<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                &JNIEnv<'env>,
            ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIFunction14<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            Res,
            Ret,
        >
        where
            Self: Sized
                + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            T13: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                arg13: T13::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                let conv_arg10: T10 = TryFromJavaValue::try_from(arg10, env)?;
                let conv_arg11: T11 = TryFromJavaValue::try_from(arg11, env)?;
                let conv_arg12: T12 = TryFromJavaValue::try_from(arg12, env)?;
                let conv_arg13: T13 = TryFromJavaValue::try_from(arg13, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        conv_arg10,
                        conv_arg11,
                        conv_arg12,
                        conv_arg13,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                arg13: T13::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            arg10,
                            arg11,
                            arg12,
                            arg13,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            Res,
            Ret,
        > JNIFunction14<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            T13: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvFunction14<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            Res,
            Ret,
        >
        where
            Self: Sized
                + Fn(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    &JNIEnv<'env>,
                ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            T13: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                arg13: T13::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                let conv_arg10: T10 = TryFromJavaValue::try_from(arg10, env)?;
                let conv_arg11: T11 = TryFromJavaValue::try_from(arg11, env)?;
                let conv_arg12: T12 = TryFromJavaValue::try_from(arg12, env)?;
                let conv_arg13: T13 = TryFromJavaValue::try_from(arg13, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        conv_arg10,
                        conv_arg11,
                        conv_arg12,
                        conv_arg13,
                        env,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                arg13: T13::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            arg10,
                            arg11,
                            arg12,
                            arg13,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            Res,
            Ret,
        > JNIEnvFunction14<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                &JNIEnv<'env>,
            ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            T13: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIFunction15<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            Res,
            Ret,
        >
        where
            Self: Sized
                + Fn(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    T14,
                ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            T13: TryFromJavaValue<'env, 'borrow>,
            T14: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                arg13: T13::Source,
                arg14: T14::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                let conv_arg10: T10 = TryFromJavaValue::try_from(arg10, env)?;
                let conv_arg11: T11 = TryFromJavaValue::try_from(arg11, env)?;
                let conv_arg12: T12 = TryFromJavaValue::try_from(arg12, env)?;
                let conv_arg13: T13 = TryFromJavaValue::try_from(arg13, env)?;
                let conv_arg14: T14 = TryFromJavaValue::try_from(arg14, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        conv_arg10,
                        conv_arg11,
                        conv_arg12,
                        conv_arg13,
                        conv_arg14,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                arg13: T13::Source,
                arg14: T14::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            arg10,
                            arg11,
                            arg12,
                            arg13,
                            arg14,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            Res,
            Ret,
        > JNIFunction15<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
            ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            T13: TryFromJavaValue<'env, 'borrow>,
            T14: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvFunction15<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            Res,
            Ret,
        >
        where
            Self: Sized
                + Fn(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    T14,
                    &JNIEnv<'env>,
                ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            T13: TryFromJavaValue<'env, 'borrow>,
            T14: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                arg13: T13::Source,
                arg14: T14::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                let conv_arg10: T10 = TryFromJavaValue::try_from(arg10, env)?;
                let conv_arg11: T11 = TryFromJavaValue::try_from(arg11, env)?;
                let conv_arg12: T12 = TryFromJavaValue::try_from(arg12, env)?;
                let conv_arg13: T13 = TryFromJavaValue::try_from(arg13, env)?;
                let conv_arg14: T14 = TryFromJavaValue::try_from(arg14, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        conv_arg10,
                        conv_arg11,
                        conv_arg12,
                        conv_arg13,
                        conv_arg14,
                        env,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                arg13: T13::Source,
                arg14: T14::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            arg10,
                            arg11,
                            arg12,
                            arg13,
                            arg14,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            Res,
            Ret,
        > JNIEnvFunction15<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                &JNIEnv<'env>,
            ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            T13: TryFromJavaValue<'env, 'borrow>,
            T14: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method ignoring
        /// self and taking N arguments, with type conversions and error
        /// handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIFunction16<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            T15,
            Res,
            Ret,
        >
        where
            Self: Sized
                + Fn(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    T14,
                    T15,
                ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            T13: TryFromJavaValue<'env, 'borrow>,
            T14: TryFromJavaValue<'env, 'borrow>,
            T15: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                arg13: T13::Source,
                arg14: T14::Source,
                arg15: T15::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                let conv_arg10: T10 = TryFromJavaValue::try_from(arg10, env)?;
                let conv_arg11: T11 = TryFromJavaValue::try_from(arg11, env)?;
                let conv_arg12: T12 = TryFromJavaValue::try_from(arg12, env)?;
                let conv_arg13: T13 = TryFromJavaValue::try_from(arg13, env)?;
                let conv_arg14: T14 = TryFromJavaValue::try_from(arg14, env)?;
                let conv_arg15: T15 = TryFromJavaValue::try_from(arg15, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        conv_arg10,
                        conv_arg11,
                        conv_arg12,
                        conv_arg13,
                        conv_arg14,
                        conv_arg15,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                arg13: T13::Source,
                arg14: T14::Source,
                arg15: T15::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            arg10,
                            arg11,
                            arg12,
                            arg13,
                            arg14,
                            arg15,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            T15,
            Res,
            Ret,
        > JNIFunction16<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            T15,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
            ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            T13: TryFromJavaValue<'env, 'borrow>,
            T14: TryFromJavaValue<'env, 'borrow>,
            T15: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
        /// Helper trait for wrapping a function as a Java method
        /// taking [`&JNIEnv`](JNIEnv), ignoring self, and taking N arguments, with
        /// type conversions and error handling.
        ///
        /// See the [`jni_function`](crate::jni_function!) macro.
        #[doc(hidden)]
        pub trait JNIEnvFunction16<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            T15,
            Res,
            Ret,
        >
        where
            Self: Sized
                + Fn(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    T14,
                    T15,
                    &JNIEnv<'env>,
                ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            T13: TryFromJavaValue<'env, 'borrow>,
            T14: TryFromJavaValue<'env, 'borrow>,
            T15: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {
            #[inline]
            fn call_convert_value(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                arg13: T13::Source,
                arg14: T14::Source,
                arg15: T15::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Result<Ret::Target, Error> {
                let conv_arg0: T0 = TryFromJavaValue::try_from(arg0, env)?;
                let conv_arg1: T1 = TryFromJavaValue::try_from(arg1, env)?;
                let conv_arg2: T2 = TryFromJavaValue::try_from(arg2, env)?;
                let conv_arg3: T3 = TryFromJavaValue::try_from(arg3, env)?;
                let conv_arg4: T4 = TryFromJavaValue::try_from(arg4, env)?;
                let conv_arg5: T5 = TryFromJavaValue::try_from(arg5, env)?;
                let conv_arg6: T6 = TryFromJavaValue::try_from(arg6, env)?;
                let conv_arg7: T7 = TryFromJavaValue::try_from(arg7, env)?;
                let conv_arg8: T8 = TryFromJavaValue::try_from(arg8, env)?;
                let conv_arg9: T9 = TryFromJavaValue::try_from(arg9, env)?;
                let conv_arg10: T10 = TryFromJavaValue::try_from(arg10, env)?;
                let conv_arg11: T11 = TryFromJavaValue::try_from(arg11, env)?;
                let conv_arg12: T12 = TryFromJavaValue::try_from(arg12, env)?;
                let conv_arg13: T13 = TryFromJavaValue::try_from(arg13, env)?;
                let conv_arg14: T14 = TryFromJavaValue::try_from(arg14, env)?;
                let conv_arg15: T15 = TryFromJavaValue::try_from(arg15, env)?;
                (self)(
                        conv_arg0,
                        conv_arg1,
                        conv_arg2,
                        conv_arg3,
                        conv_arg4,
                        conv_arg5,
                        conv_arg6,
                        conv_arg7,
                        conv_arg8,
                        conv_arg9,
                        conv_arg10,
                        conv_arg11,
                        conv_arg12,
                        conv_arg13,
                        conv_arg14,
                        conv_arg15,
                        env,
                    )
                    .into_return_value(env)
            }
            #[inline]
            unsafe fn call_handle_error(
                self,
                arg0: T0::Source,
                arg1: T1::Source,
                arg2: T2::Source,
                arg3: T3::Source,
                arg4: T4::Source,
                arg5: T5::Source,
                arg6: T6::Source,
                arg7: T7::Source,
                arg8: T8::Source,
                arg9: T9::Source,
                arg10: T10::Source,
                arg11: T11::Source,
                arg12: T12::Source,
                arg13: T13::Source,
                arg14: T14::Source,
                arg15: T15::Source,
                env: &'borrow JNIEnv<'env>,
            ) -> Ret::Target {
                let res = match std::panic::catch_unwind(
                    AssertUnwindSafe(|| {
                        self.call_convert_value(
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                            arg8,
                            arg9,
                            arg10,
                            arg11,
                            arg12,
                            arg13,
                            arg14,
                            arg15,
                            env,
                        )
                    }),
                ) {
                    Ok(v) => v,
                    Err(e) => Error::from_panic(e, env),
                };
                match res {
                    Ok(v) => v,
                    Err(e) => raise(e, env),
                }
            }
        }
        impl<
            'env: 'borrow,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            T15,
            Res,
            Ret,
        > JNIEnvFunction16<
            'env,
            'borrow,
            Func,
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            T15,
            Res,
            Ret,
        > for Func
        where
            Func: Fn(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                &JNIEnv<'env>,
            ) -> Res,
            T0: TryFromJavaValue<'env, 'borrow>,
            T1: TryFromJavaValue<'env, 'borrow>,
            T2: TryFromJavaValue<'env, 'borrow>,
            T3: TryFromJavaValue<'env, 'borrow>,
            T4: TryFromJavaValue<'env, 'borrow>,
            T5: TryFromJavaValue<'env, 'borrow>,
            T6: TryFromJavaValue<'env, 'borrow>,
            T7: TryFromJavaValue<'env, 'borrow>,
            T8: TryFromJavaValue<'env, 'borrow>,
            T9: TryFromJavaValue<'env, 'borrow>,
            T10: TryFromJavaValue<'env, 'borrow>,
            T11: TryFromJavaValue<'env, 'borrow>,
            T12: TryFromJavaValue<'env, 'borrow>,
            T13: TryFromJavaValue<'env, 'borrow>,
            T14: TryFromJavaValue<'env, 'borrow>,
            T15: TryFromJavaValue<'env, 'borrow>,
            Res: ReturnValue<'env, Ret>,
            Ret: TryIntoJavaValue<'env>,
        {}
    }
    pub use jni_function::*;
}
pub use java::*;
pub mod jruby {
    pub mod sys {
        pub mod java {
            pub mod org {
                pub mod jruby {
                    pub mod runtime {
                        mod thread_context {
                            mod bridge_mod {
                                #![allow(non_snake_case)]
                                use robusta_jni::{
                                    convert::{Signature, TryFromJavaValue, TryIntoJavaValue},
                                    jni::errors::Result as JniResult, jni::objects::AutoLocal,
                                    jni::JNIEnv,
                                };
                                use crate::jruby::sys::java::org::jruby::Ruby;
                                #[package(org.jruby.runtime)]
                                pub struct ThreadContext<'env: 'borrow, 'borrow> {
                                    #[instance]
                                    raw: AutoLocal<'env, 'borrow>,
                                }
                                #[automatically_derived]
                                impl<
                                    'env: 'borrow,
                                    'borrow,
                                > ::robusta_jni::convert::Signature
                                for ThreadContext<'env, 'borrow> {
                                    const SIG_TYPE: &'static str = "Lorg/jruby/runtime/ThreadContext;";
                                }
                                #[automatically_derived]
                                impl<
                                    'env: 'borrow,
                                    'borrow,
                                > ::robusta_jni::convert::Signature
                                for &ThreadContext<'env, 'borrow> {
                                    const SIG_TYPE: &'static str = <ThreadContext as ::robusta_jni::convert::Signature>::SIG_TYPE;
                                }
                                #[automatically_derived]
                                impl<
                                    'env: 'borrow,
                                    'borrow,
                                > ::robusta_jni::convert::Signature
                                for &mut ThreadContext<'env, 'borrow> {
                                    const SIG_TYPE: &'static str = <ThreadContext as ::robusta_jni::convert::Signature>::SIG_TYPE;
                                }
                                const _: fn() = || {
                                    {
                                        trait TypeEq {
                                            type This: ?Sized;
                                        }
                                        impl<T: ?Sized> TypeEq for T {
                                            type This = Self;
                                        }
                                        fn assert_type_eq_all<T, U>()
                                        where
                                            T: ?Sized + TypeEq<This = U>,
                                            U: ?Sized,
                                        {}
                                        assert_type_eq_all::<
                                            AutoLocal<'static, 'static>,
                                            ::robusta_jni::jni::objects::AutoLocal<'static, 'static>,
                                        >();
                                    }
                                };
                                #[automatically_derived]
                                impl<
                                    'env: 'borrow,
                                    'borrow,
                                > ::robusta_jni::convert::TryIntoJavaValue<'env>
                                for ThreadContext<'env, 'borrow> {
                                    type Target = ::robusta_jni::jni::objects::JObject<'env>;
                                    fn try_into(
                                        self,
                                        env: &::robusta_jni::jni::JNIEnv<'env>,
                                    ) -> ::robusta_jni::jni::errors::Result<Self::Target> {
                                        ::robusta_jni::convert::TryIntoJavaValue::try_into(
                                            self,
                                            env,
                                        )
                                    }
                                }
                                #[automatically_derived]
                                impl<
                                    'env: 'borrow,
                                    'borrow,
                                > ::robusta_jni::convert::TryIntoJavaValue<'env>
                                for &ThreadContext<'env, 'borrow> {
                                    type Target = ::robusta_jni::jni::objects::JObject<'env>;
                                    fn try_into(
                                        self,
                                        env: &::robusta_jni::jni::JNIEnv<'env>,
                                    ) -> ::robusta_jni::jni::errors::Result<Self::Target> {
                                        Ok(self.raw.as_obj())
                                    }
                                }
                                #[automatically_derived]
                                impl<
                                    'env: 'borrow,
                                    'borrow,
                                > ::robusta_jni::convert::TryIntoJavaValue<'env>
                                for &mut ThreadContext<'env, 'borrow> {
                                    type Target = ::robusta_jni::jni::objects::JObject<'env>;
                                    fn try_into(
                                        self,
                                        env: &::robusta_jni::jni::JNIEnv<'env>,
                                    ) -> ::robusta_jni::jni::errors::Result<Self::Target> {
                                        ::robusta_jni::convert::TryIntoJavaValue::try_into(
                                            self,
                                            env,
                                        )
                                    }
                                }
                                const _: fn() = || {
                                    {
                                        trait TypeEq {
                                            type This: ?Sized;
                                        }
                                        impl<T: ?Sized> TypeEq for T {
                                            type This = Self;
                                        }
                                        fn assert_type_eq_all<T, U>()
                                        where
                                            T: ?Sized + TypeEq<This = U>,
                                            U: ?Sized,
                                        {}
                                        assert_type_eq_all::<
                                            AutoLocal<'static, 'static>,
                                            ::robusta_jni::jni::objects::AutoLocal<'static, 'static>,
                                        >();
                                    }
                                };
                                #[automatically_derived]
                                impl<
                                    'env: 'borrow,
                                    'borrow,
                                > ::robusta_jni::convert::TryFromJavaValue<'env, 'borrow>
                                for ThreadContext<'env, 'borrow> {
                                    type Source = ::robusta_jni::jni::objects::JObject<'env>;
                                    fn try_from(
                                        source: Self::Source,
                                        env: &'borrow ::robusta_jni::jni::JNIEnv<'env>,
                                    ) -> ::robusta_jni::jni::errors::Result<Self> {
                                        Ok(Self {
                                            raw: ::robusta_jni::jni::objects::AutoLocal::new(
                                                env,
                                                source,
                                            ),
                                        })
                                    }
                                }
                                impl<'env: 'borrow, 'borrow> ThreadContext<'env, 'borrow> {
                                    pub fn getRuntime(
                                        &self,
                                        env: &'borrow JNIEnv<'env>,
                                    ) -> JniResult<Ruby> {
                                        let env: &'_ ::robusta_jni::jni::JNIEnv<'_> = env;
                                        let res = env
                                            .call_method(
                                                ::robusta_jni::convert::JavaValue::autobox(
                                                    ::robusta_jni::convert::TryIntoJavaValue::try_into(
                                                        self,
                                                        &env,
                                                    )?,
                                                    &env,
                                                ),
                                                "getRuntime",
                                                [
                                                    "(",
                                                    ")",
                                                    <Ruby as ::robusta_jni::convert::TryIntoJavaValue>::SIG_TYPE,
                                                ]
                                                    .join(""),
                                                &[],
                                            );
                                        res.and_then(|v| ::std::convert::TryInto::try_into(
                                                ::robusta_jni::convert::JValueWrapper::from(v),
                                            ))
                                            .and_then(|v| ::robusta_jni::convert::TryFromJavaValue::try_from(
                                                v,
                                                &env,
                                            ))
                                    }
                                }
                            }
                            pub use bridge_mod::*;
                        }
                        pub use thread_context::*;
                    }
                    mod ruby_module {
                        mod bridge_mod {
                            #![allow(non_snake_case)]
                            use robusta_jni::{
                                convert::{Signature, TryFromJavaValue, TryIntoJavaValue},
                                jni::errors::Result as JniResult, jni::objects::AutoLocal,
                                jni::JNIEnv, jni::objects::JObject,
                            };
                            #[package(org.jruby)]
                            pub struct RubyModule<'env: 'borrow, 'borrow> {
                                #[instance]
                                raw: AutoLocal<'env, 'borrow>,
                            }
                            #[automatically_derived]
                            impl<
                                'env: 'borrow,
                                'borrow,
                            > ::robusta_jni::convert::Signature
                            for RubyModule<'env, 'borrow> {
                                const SIG_TYPE: &'static str = "Lorg/jruby/RubyModule;";
                            }
                            #[automatically_derived]
                            impl<
                                'env: 'borrow,
                                'borrow,
                            > ::robusta_jni::convert::Signature
                            for &RubyModule<'env, 'borrow> {
                                const SIG_TYPE: &'static str = <RubyModule as ::robusta_jni::convert::Signature>::SIG_TYPE;
                            }
                            #[automatically_derived]
                            impl<
                                'env: 'borrow,
                                'borrow,
                            > ::robusta_jni::convert::Signature
                            for &mut RubyModule<'env, 'borrow> {
                                const SIG_TYPE: &'static str = <RubyModule as ::robusta_jni::convert::Signature>::SIG_TYPE;
                            }
                            const _: fn() = || {
                                {
                                    trait TypeEq {
                                        type This: ?Sized;
                                    }
                                    impl<T: ?Sized> TypeEq for T {
                                        type This = Self;
                                    }
                                    fn assert_type_eq_all<T, U>()
                                    where
                                        T: ?Sized + TypeEq<This = U>,
                                        U: ?Sized,
                                    {}
                                    assert_type_eq_all::<
                                        AutoLocal<'static, 'static>,
                                        ::robusta_jni::jni::objects::AutoLocal<'static, 'static>,
                                    >();
                                }
                            };
                            #[automatically_derived]
                            impl<
                                'env: 'borrow,
                                'borrow,
                            > ::robusta_jni::convert::TryIntoJavaValue<'env>
                            for RubyModule<'env, 'borrow> {
                                type Target = ::robusta_jni::jni::objects::JObject<'env>;
                                fn try_into(
                                    self,
                                    env: &::robusta_jni::jni::JNIEnv<'env>,
                                ) -> ::robusta_jni::jni::errors::Result<Self::Target> {
                                    ::robusta_jni::convert::TryIntoJavaValue::try_into(
                                        self,
                                        env,
                                    )
                                }
                            }
                            #[automatically_derived]
                            impl<
                                'env: 'borrow,
                                'borrow,
                            > ::robusta_jni::convert::TryIntoJavaValue<'env>
                            for &RubyModule<'env, 'borrow> {
                                type Target = ::robusta_jni::jni::objects::JObject<'env>;
                                fn try_into(
                                    self,
                                    env: &::robusta_jni::jni::JNIEnv<'env>,
                                ) -> ::robusta_jni::jni::errors::Result<Self::Target> {
                                    Ok(self.raw.as_obj())
                                }
                            }
                            #[automatically_derived]
                            impl<
                                'env: 'borrow,
                                'borrow,
                            > ::robusta_jni::convert::TryIntoJavaValue<'env>
                            for &mut RubyModule<'env, 'borrow> {
                                type Target = ::robusta_jni::jni::objects::JObject<'env>;
                                fn try_into(
                                    self,
                                    env: &::robusta_jni::jni::JNIEnv<'env>,
                                ) -> ::robusta_jni::jni::errors::Result<Self::Target> {
                                    ::robusta_jni::convert::TryIntoJavaValue::try_into(
                                        self,
                                        env,
                                    )
                                }
                            }
                            const _: fn() = || {
                                {
                                    trait TypeEq {
                                        type This: ?Sized;
                                    }
                                    impl<T: ?Sized> TypeEq for T {
                                        type This = Self;
                                    }
                                    fn assert_type_eq_all<T, U>()
                                    where
                                        T: ?Sized + TypeEq<This = U>,
                                        U: ?Sized,
                                    {}
                                    assert_type_eq_all::<
                                        AutoLocal<'static, 'static>,
                                        ::robusta_jni::jni::objects::AutoLocal<'static, 'static>,
                                    >();
                                }
                            };
                            #[automatically_derived]
                            impl<
                                'env: 'borrow,
                                'borrow,
                            > ::robusta_jni::convert::TryFromJavaValue<'env, 'borrow>
                            for RubyModule<'env, 'borrow> {
                                type Source = ::robusta_jni::jni::objects::JObject<'env>;
                                fn try_from(
                                    source: Self::Source,
                                    env: &'borrow ::robusta_jni::jni::JNIEnv<'env>,
                                ) -> ::robusta_jni::jni::errors::Result<Self> {
                                    Ok(Self {
                                        raw: ::robusta_jni::jni::objects::AutoLocal::new(
                                            env,
                                            source,
                                        ),
                                    })
                                }
                            }
                            impl<'env: 'borrow, 'borrow> RubyModule<'env, 'borrow> {
                                pub fn defineAnnotatedMethods(
                                    &self,
                                    env: &JNIEnv,
                                    cls: JObject,
                                ) -> JniResult<()> {
                                    let env: &'_ ::robusta_jni::jni::JNIEnv<'_> = env;
                                    let res = env
                                        .call_method(
                                            ::robusta_jni::convert::JavaValue::autobox(
                                                ::robusta_jni::convert::TryIntoJavaValue::try_into(
                                                    self,
                                                    &env,
                                                )?,
                                                &env,
                                            ),
                                            "defineAnnotatedMethods",
                                            [
                                                "(",
                                                "Ljava/lang/Class;",
                                                ")",
                                                <() as ::robusta_jni::convert::TryIntoJavaValue>::SIG_TYPE,
                                            ]
                                                .join(""),
                                            &[
                                                ::std::convert::Into::into(
                                                    <JObject as ::robusta_jni::convert::TryIntoJavaValue>::try_into(
                                                        cls,
                                                        &env,
                                                    )?,
                                                ),
                                            ],
                                        );
                                    res.and_then(|v| ::std::convert::TryInto::try_into(
                                            ::robusta_jni::convert::JValueWrapper::from(v),
                                        ))
                                        .and_then(|v| ::robusta_jni::convert::TryFromJavaValue::try_from(
                                            v,
                                            &env,
                                        ))
                                }
                            }
                        }
                        pub use bridge_mod::*;
                    }
                    pub use ruby_module::*;
                    mod ruby {
                        mod bridge_mod {
                            #![allow(non_snake_case)]
                            use robusta_jni::{
                                convert::{Signature, TryFromJavaValue, TryIntoJavaValue},
                                jni::errors::Result as JniResult, jni::objects::AutoLocal,
                                jni::JNIEnv,
                            };
                            #[package(org.jruby)]
                            pub struct Ruby<'env: 'borrow, 'borrow> {
                                #[instance]
                                raw: AutoLocal<'env, 'borrow>,
                            }
                            #[automatically_derived]
                            impl<
                                'env: 'borrow,
                                'borrow,
                            > ::robusta_jni::convert::Signature for Ruby<'env, 'borrow> {
                                const SIG_TYPE: &'static str = "Lorg/jruby/Ruby;";
                            }
                            #[automatically_derived]
                            impl<
                                'env: 'borrow,
                                'borrow,
                            > ::robusta_jni::convert::Signature
                            for &Ruby<'env, 'borrow> {
                                const SIG_TYPE: &'static str = <Ruby as ::robusta_jni::convert::Signature>::SIG_TYPE;
                            }
                            #[automatically_derived]
                            impl<
                                'env: 'borrow,
                                'borrow,
                            > ::robusta_jni::convert::Signature
                            for &mut Ruby<'env, 'borrow> {
                                const SIG_TYPE: &'static str = <Ruby as ::robusta_jni::convert::Signature>::SIG_TYPE;
                            }
                            const _: fn() = || {
                                {
                                    trait TypeEq {
                                        type This: ?Sized;
                                    }
                                    impl<T: ?Sized> TypeEq for T {
                                        type This = Self;
                                    }
                                    fn assert_type_eq_all<T, U>()
                                    where
                                        T: ?Sized + TypeEq<This = U>,
                                        U: ?Sized,
                                    {}
                                    assert_type_eq_all::<
                                        AutoLocal<'static, 'static>,
                                        ::robusta_jni::jni::objects::AutoLocal<'static, 'static>,
                                    >();
                                }
                            };
                            #[automatically_derived]
                            impl<
                                'env: 'borrow,
                                'borrow,
                            > ::robusta_jni::convert::TryIntoJavaValue<'env>
                            for Ruby<'env, 'borrow> {
                                type Target = ::robusta_jni::jni::objects::JObject<'env>;
                                fn try_into(
                                    self,
                                    env: &::robusta_jni::jni::JNIEnv<'env>,
                                ) -> ::robusta_jni::jni::errors::Result<Self::Target> {
                                    ::robusta_jni::convert::TryIntoJavaValue::try_into(
                                        self,
                                        env,
                                    )
                                }
                            }
                            #[automatically_derived]
                            impl<
                                'env: 'borrow,
                                'borrow,
                            > ::robusta_jni::convert::TryIntoJavaValue<'env>
                            for &Ruby<'env, 'borrow> {
                                type Target = ::robusta_jni::jni::objects::JObject<'env>;
                                fn try_into(
                                    self,
                                    env: &::robusta_jni::jni::JNIEnv<'env>,
                                ) -> ::robusta_jni::jni::errors::Result<Self::Target> {
                                    Ok(self.raw.as_obj())
                                }
                            }
                            #[automatically_derived]
                            impl<
                                'env: 'borrow,
                                'borrow,
                            > ::robusta_jni::convert::TryIntoJavaValue<'env>
                            for &mut Ruby<'env, 'borrow> {
                                type Target = ::robusta_jni::jni::objects::JObject<'env>;
                                fn try_into(
                                    self,
                                    env: &::robusta_jni::jni::JNIEnv<'env>,
                                ) -> ::robusta_jni::jni::errors::Result<Self::Target> {
                                    ::robusta_jni::convert::TryIntoJavaValue::try_into(
                                        self,
                                        env,
                                    )
                                }
                            }
                            const _: fn() = || {
                                {
                                    trait TypeEq {
                                        type This: ?Sized;
                                    }
                                    impl<T: ?Sized> TypeEq for T {
                                        type This = Self;
                                    }
                                    fn assert_type_eq_all<T, U>()
                                    where
                                        T: ?Sized + TypeEq<This = U>,
                                        U: ?Sized,
                                    {}
                                    assert_type_eq_all::<
                                        AutoLocal<'static, 'static>,
                                        ::robusta_jni::jni::objects::AutoLocal<'static, 'static>,
                                    >();
                                }
                            };
                            #[automatically_derived]
                            impl<
                                'env: 'borrow,
                                'borrow,
                            > ::robusta_jni::convert::TryFromJavaValue<'env, 'borrow>
                            for Ruby<'env, 'borrow> {
                                type Source = ::robusta_jni::jni::objects::JObject<'env>;
                                fn try_from(
                                    source: Self::Source,
                                    env: &'borrow ::robusta_jni::jni::JNIEnv<'env>,
                                ) -> ::robusta_jni::jni::errors::Result<Self> {
                                    Ok(Self {
                                        raw: ::robusta_jni::jni::objects::AutoLocal::new(
                                            env,
                                            source,
                                        ),
                                    })
                                }
                            }
                        }
                        pub use bridge_mod::*;
                    }
                    pub use ruby::*;
                }
            }
        }
    }
}
/// The possible types of [`Error`].
pub enum ErrorType {
    /// An error generated in Rust code that will raise an exception when
    /// returned to Java.
    Error(&'static str, Cow<'static, str>),
    /// A Java Exception captured from Java as an Error.
    Exception(robusta_jni::jni::errors::Error),
}
#[automatically_derived]
impl ::core::fmt::Debug for ErrorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            ErrorType::Error(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "Error",
                    __self_0,
                    &__self_1,
                )
            }
            ErrorType::Exception(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Exception",
                    &__self_0,
                )
            }
        }
    }
}
/// Wrapper type for Java `Exception`s or other interrupts.
pub struct Error(ErrorType);
#[automatically_derived]
impl ::core::fmt::Debug for Error {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Error", &&self.0)
    }
}
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
    match e.0 {
        ErrorType::Exception(jni_err) => {
            let _ = env
                .throw_new(
                    "java/lang/RuntimeException",
                    {
                        let res = ::alloc::fmt::format(format_args!("{0:?}", jni_err));
                        res
                    },
                );
        }
        ErrorType::Error(cls, msg) => {
            let _ = env.throw_new(cls, msg);
        }
    };
    unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
}
pub trait ReturnValue<'env, T>
where
    T: TryIntoJavaValue<'env>,
{
    fn into_return_value(
        self,
        env: &JNIEnv<'env>,
    ) -> Result<<T as TryIntoJavaValue<'env>>::Target, Error>;
}
impl<'env, T> ReturnValue<'env, T> for Result<T, Error>
where
    T: TryIntoJavaValue<'env>,
{
    fn into_return_value(
        self,
        env: &JNIEnv<'env>,
    ) -> Result<<T as TryIntoJavaValue<'env>>::Target, Error> {
        match self {
            Ok(val) => Ok(TryIntoJavaValue::try_into(val, env)?),
            Err(err) => Err(err),
        }
    }
}
impl<'env, T> ReturnValue<'env, T> for T
where
    T: TryIntoJavaValue<'env>,
{
    fn into_return_value(
        self,
        env: &JNIEnv<'env>,
    ) -> Result<<T as TryIntoJavaValue<'env>>::Target, Error> {
        Ok(TryIntoJavaValue::try_into(self, env)?)
    }
}
