#[robusta_jni::bridge]
mod bridge_mod {
    use robusta_jni::{
        convert::{
            Signature, TryFromJavaValue, TryIntoJavaValue,
        },
        jni::objects::AutoLocal,
    };

    #[derive(Signature, TryIntoJavaValue, TryFromJavaValue)]
    #[package(org.jruby)]
    pub struct Ruby<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>,
    }
}

pub use bridge_mod::*;
