pub mod errors {
    use std::borrow::Cow;

    #[derive(Debug)]
    pub enum Error {
        /// An error generated in Rust code that will raise an exception when
        /// returned to Java.
        /// Todo: change first arg type to "Throwable"
        Error(Cow<'static, str>, Cow<'static, str>),
    }

    impl Error {
        pub fn new<F, T>(class: F, msg: T) -> Self
            where
                F: Into<Cow<'static, str>>,
                T: Into<Cow<'static, str>>,
        {
            Self::Error(class.into(), msg.into())
        }
    }

    pub fn type_error(msg: String) -> Error {
        Error::new("java/lang/RuntimeException", msg)
    }

    pub fn runtime_error(msg: String) -> Error {
        Error::new("java/lang/ClassCastException", msg)
    }
}

pub mod args {
    use jni::JNIEnv;
    use jni::objects::JString;
    use jni::sys::{jobject, jstring};
    use crate::implementation::errors::{Error, runtime_error};

    type JHash = jobject;

    pub struct Args {
        pub str_arg: String,
        opts: Option<jobject>,
    }

    pub struct Opts {
        opts_hash: JHash,
    }

    impl Args {
        pub fn new(mut env: JNIEnv, str: jstring, opts: Option<jobject>) -> Result<Args, Error> {
            let j_str = unsafe { JString::from_raw(str) };
            let j_res = env.get_string(&j_str);
            match j_res {
                Ok(java_str) => { Ok(Args { str_arg: java_str.into(), opts: opts.into() }) }
                Err(err) => { Err(runtime_error("".to_string())) }
            }
        }

        pub fn opts(&self) -> Option<Opts> {
            self.opts.map(|opts_hash| Opts { opts_hash })
        }
    }

    impl Opts {
        pub fn lookup<U>(&self, _key: &str) -> Result<Option<U>, Error> {
            Err(runtime_error("Not implemented".to_string()))
        }

        pub fn lookup_str(&self, _key: &str) -> Result<Option<String>, Error> {
            Err(runtime_error("Not implemented".to_string()))
        }
    }
}
