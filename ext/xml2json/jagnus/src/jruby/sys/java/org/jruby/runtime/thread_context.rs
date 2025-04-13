#[robusta_jni::bridge]
mod bridge_mod {
    use robusta_jni::{
        convert::{
            Signature, TryFromJavaValue, TryIntoJavaValue,
        },
        jni::errors::Result as JniResult,
        jni::objects::AutoLocal,
        jni::JNIEnv,
    };
    use crate::jruby::sys::java::org::jruby::Ruby;

    #[derive(Signature, TryIntoJavaValue, TryFromJavaValue)]
    #[package(org.jruby.runtime)]
    pub struct ThreadContext<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>,
    }

    impl<'env: 'borrow, 'borrow> ThreadContext<'env, 'borrow> {
        pub extern "java" fn getRuntime(&self,
                                        env: &'borrow JNIEnv<'env>,
        ) -> JniResult<Ruby> {}
    }
}

pub use bridge_mod::*;
