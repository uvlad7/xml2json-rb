#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[cfg(feature = "jruby")]
use std::os::raw::c_void;
#[cfg(feature = "jruby")]
use robusta_jni::jni::{
    JavaVM, JNIEnv, NativeMethod, objects::JClass, strings::JNIString,
};
#[cfg(feature = "jruby")]
use robusta_jni::jni::objects::{JObject, JValue};
#[cfg(feature = "jruby")]
use robusta_jni::jni::sys::{jint, JNI_ERR, JNI_VERSION_1_4};
#[cfg(feature = "jruby")]
use crate::jni::Error;
#[cfg(feature = "jruby")]
mod jni {
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
        fn from_panic<'local>(
            e: Box<dyn Any + Send + 'static>,
            env: &JNIEnv<'local>,
        ) -> ! {
            let msg: Cow<'static, str> = if let Some(&m) = e
                .downcast_ref::<&'static str>()
            {
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
                let _ = env
                    .throw_new(
                        "java/lang/RuntimeException",
                        {
                            let res = ::alloc::fmt::format(
                                format_args!("{0:?}", jni_err),
                            );
                            res
                        },
                    );
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
    #[allow(missing_docs)]
    impl<'env: 'borrow, 'borrow, Func, A, Res, Ret> Function1<Func, A, Res, Ret>
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
        unsafe fn call_convert_value(
            self,
            a: A::Source,
            env: &'borrow JNIEnv<'env>,
        ) -> Result<Ret::Target, Error> {
            let arg: A = TryFromJavaValue::try_from(a, env)?;
            let res = (self.func)(arg);
            res.into_return_value(env)
        }
        #[inline]
        pub unsafe fn call_handle_error(
            self,
            a: A::Source,
            env: &'borrow JNIEnv<'env>,
        ) -> Ret::Target {
            let res = match std::panic::catch_unwind(
                AssertUnwindSafe(|| self.call_convert_value(a, env)),
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
    mod java {
        pub mod org {
            pub mod jruby {
                use robusta_jni::bridge;
                mod ruby_module {
                    #![allow(non_snake_case)]
                    use robusta_jni::convert::{
                        Signature, TryFromJavaValue, TryIntoJavaValue,
                    };
                    use robusta_jni::jni::errors::Error as JniError;
                    use robusta_jni::jni::errors::Result as JniResult;
                    use robusta_jni::jni::objects::AutoLocal;
                    use robusta_jni::jni::JNIEnv;
                    use robusta_jni::jni::objects::JObject;
                    #[package(org.jruby)]
                    pub struct RubyModule<'env: 'borrow, 'borrow> {
                        #[instance]
                        raw: AutoLocal<'env, 'borrow>,
                    }
                    #[automatically_derived]
                    impl<'env: 'borrow, 'borrow> ::robusta_jni::convert::Signature
                    for RubyModule<'env, 'borrow> {
                        const SIG_TYPE: &'static str = "Lorg/jruby/RubyModule;";
                    }
                    #[automatically_derived]
                    impl<'env: 'borrow, 'borrow> ::robusta_jni::convert::Signature
                    for &RubyModule<'env, 'borrow> {
                        const SIG_TYPE: &'static str = <RubyModule as ::robusta_jni::convert::Signature>::SIG_TYPE;
                    }
                    #[automatically_derived]
                    impl<'env: 'borrow, 'borrow> ::robusta_jni::convert::Signature
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
                            ::robusta_jni::convert::TryIntoJavaValue::try_into(self, env)
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
                            ::robusta_jni::convert::TryIntoJavaValue::try_into(self, env)
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
                pub use ruby_module::RubyModule;
            }
        }
    }
    pub use java::org::jruby::RubyModule;
}
/// use rb_sys::set_global_tracking_allocator;
///
/// set_global_tracking_allocator!();
use xml2json_rs::{
    XmlBuilder, JsonBuilder, JsonConfig, XmlConfig, Declaration, Version, Encoding,
    Indentation,
};
#[cfg(feature = "jruby")]
fn build_xml(json_s: String) -> Result<String, Error> {
    let mut xml_builder: XmlBuilder = XmlBuilder::default();
    Ok(xml_builder.build_from_json_string(&json_s).unwrap())
}
#[cfg(feature = "jruby")]
extern "system" fn basic_load<'local>(
    mut env: JNIEnv<'local>,
    _this: JObject<'local>,
    ruby: JObject<'local>,
) -> <bool as robusta_jni::convert::TryIntoJavaValue<'local>>::Target {
    let fail: <bool as robusta_jni::convert::TryIntoJavaValue<'local>>::Target = robusta_jni::jni::sys::JNI_FALSE;
    let Ok(module_name) = env.new_string("Xml2Json") else {
        return fail;
    };
    let Ok(module_val) = env
        .call_method(
            ruby,
            "defineModule",
            "(Ljava/lang/String;)Lorg/jruby/RubyModule;",
            &[JValue::Object(JObject::from(module_name))],
        ) else {
        return fail;
    };
    let JValue::Object(module) = module_val else {
        return fail;
    };
    let Ok(xml_name) = env.new_string("Xml") else {
        return fail;
    };
    let Ok(xml_val) = env
        .call_method(
            module,
            "defineModuleUnder",
            "(Ljava/lang/String;)Lorg/jruby/RubyModule;",
            &[JValue::Object(JObject::from(xml_name))],
        ) else {
        return fail;
    };
    let JValue::Object(xml) = xml_val else {
        return fail;
    };
    let Ok(clazz) = env.find_class("io/github/uvlad7/xml2json/Xml") else {
        return fail;
    };
    let a: jni::RubyModule = robusta_jni::convert::TryFromJavaValue::try_from(xml, &env)
        .unwrap();
    let Ok(_) = a.defineAnnotatedMethods(&env, JObject::from(clazz)) else {
        return fail;
    };
    robusta_jni::jni::sys::JNI_TRUE
}
#[cfg(feature = "jruby")]
/// This function is executed on loading native library by JVM.
/// It initializes the cache of method and class references.
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn JNI_OnLoad<'local>(vm: JavaVM, _: *mut c_void) -> jint {
    let Ok(mut env) = vm.get_env() else {
        return JNI_ERR;
    };
    let Ok(serv_clazz) = env.find_class("io/github/uvlad7/xml2json/Xml2JsonService")
    else {
        return JNI_ERR;
    };
    let basic_load_func = basic_load
        as unsafe extern "system" fn(
            env: JNIEnv<'local>,
            _this: JObject<'local>,
            ruby: JObject<'local>,
        ) -> <bool as robusta_jni::convert::TryIntoJavaValue<'local>>::Target;
    let basic_load_ptr = basic_load_func as *mut c_void;
    let basic_load_method = NativeMethod {
        name: JNIString::from("basicLoad"),
        sig: JNIString::from({
            let res = ::alloc::fmt::format(
                format_args!("({0}){1}", "Lorg/jruby/Ruby;", "Z"),
            );
            res
        }),
        fn_ptr: basic_load_ptr,
    };
    let Ok(_) = env.register_native_methods(serv_clazz, &[basic_load_method]) else {
        return JNI_ERR;
    };
    let Ok(clazz) = env.find_class("io/github/uvlad7/xml2json/Xml") else {
        return JNI_ERR;
    };
    let Ok(_) = env
        .register_native_methods(
            clazz,
            &[
                {
                    unsafe extern "system" fn anon<'local>(
                        mut env: JNIEnv<'local>,
                        _class: JClass<'local>,
                        a: <String as robusta_jni::convert::TryFromJavaValue<
                            'local,
                            'local,
                        >>::Source,
                    ) -> <String as robusta_jni::convert::TryIntoJavaValue<
                        'local,
                    >>::Target {
                        crate::jni::Function1::new(build_xml).call_handle_error(a, &env)
                    }
                    let anon_func = anon
                        as unsafe extern "system" fn(
                            env: JNIEnv<'local>,
                            _class: JClass<'local>,
                            a: <String as robusta_jni::convert::TryFromJavaValue<
                                'local,
                                'local,
                            >>::Source,
                        ) -> <String as robusta_jni::convert::TryIntoJavaValue<
                            'local,
                        >>::Target;
                    let anon_ptr = anon_func as *mut c_void;
                    NativeMethod {
                        name: JNIString::from("buildNative"),
                        sig: JNIString::from({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "({0}){1}",
                                    <<String as robusta_jni::convert::TryFromJavaValue<
                                        'local,
                                        'local,
                                    >>::Source as robusta_jni::convert::Signature>::SIG_TYPE,
                                    <<String as robusta_jni::convert::TryIntoJavaValue<
                                        'local,
                                    >>::Target as robusta_jni::convert::Signature>::SIG_TYPE,
                                ),
                            );
                            res
                        }),
                        fn_ptr: anon_ptr,
                    }
                },
            ],
        ) else {
        return JNI_ERR;
    };
    JNI_VERSION_1_4
}
