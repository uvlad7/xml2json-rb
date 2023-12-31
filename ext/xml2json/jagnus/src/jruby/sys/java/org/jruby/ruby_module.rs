#[robusta_jni::bridge]
mod bridge_mod {
    use robusta_jni::{
        convert::{
            Signature, TryFromJavaValue, TryIntoJavaValue,
        },
        jni::errors::Result as JniResult,
        jni::objects::AutoLocal,
        jni::JNIEnv,
        jni::objects::JObject,
    };

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
}

pub use bridge_mod::*;
