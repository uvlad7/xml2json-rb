#[robusta_jni::bridge]
mod ruby_module {
    use robusta_jni::convert::{
        Signature, TryFromJavaValue, TryIntoJavaValue,
    };
    use robusta_jni::jni::errors::Error as JniError;
    use robusta_jni::jni::errors::Result as JniResult;
    use robusta_jni::jni::objects::AutoLocal;
    use robusta_jni::jni::JNIEnv;
    use robusta_jni::jni::objects::{JObject, JClass};

    #[derive(Signature, TryIntoJavaValue, TryFromJavaValue)]
    #[package(org.jruby)]
    pub struct RubyModule<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>,
    }

    impl<'env: 'borrow, 'borrow> RubyModule<'env, 'borrow> {
        pub extern "java" fn defineAnnotatedMethods(&self,
                                                    env: &JNIEnv,
                                                    #[input_type("Ljava/lang/Class;")] cls: JObject,
        ) -> JniResult<()> {}
    }

    #[derive(Signature, TryIntoJavaValue, TryFromJavaValue)]
    #[package(org.jruby)]
    pub struct Ruby<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>,
    }
}

pub use ruby_module::*;
