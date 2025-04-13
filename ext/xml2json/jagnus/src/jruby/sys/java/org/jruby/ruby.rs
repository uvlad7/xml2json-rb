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

    #[derive(Signature, TryIntoJavaValue, TryFromJavaValue)]
    #[package(org.jruby)]
    pub struct Ruby<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>,
    }

    // impl<'env: 'borrow, 'borrow> Ruby<'env, 'borrow> {
    //     pub extern "java" fn newArgumentError(&self,
    //                                           env: &JNIEnv,
    //                                           got: i32, min: i32, max: i32,
    //     ) -> JniResult<RaiseException> {}
    // }
}

pub use bridge_mod::*;
