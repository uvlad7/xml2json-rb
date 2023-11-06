pub mod errors {
    pub type Error = magnus::Error;

    pub fn type_error(msg: String) -> Error {
        Error::new(magnus::exception::type_error(), msg)
    }

    pub fn runtime_error(msg: String) -> Error {
        Error::new(magnus::exception::runtime_error(), msg)
    }
}

pub mod args {
    use magnus::scan_args::scan_args;
    use magnus::{RHash, Value};
    use crate::implementation::errors::Error;

    pub struct Args {
        pub str_arg: String,
        opts: Option<RHash>,
    }

    pub struct Opts {
        opts_hash: RHash,
    }

    impl Args {
        pub fn new(args: &[Value]) -> Result<Args, Error> {
            // https://docs.rs/magnus/latest/magnus/scan_args/fn.scan_args.html
            let args = scan_args::<_, _, (), (), (), ()>(args)?;
            let (json_s, ): (String, ) = args.required;
            let (opts, ): (Option<RHash>, ) = args.optional;
            Ok(Args { str_arg: json_s, opts })
        }

        pub fn opts(&self) -> Option<Opts> {
            self.opts.map(|opts_hash| Opts { opts_hash })
        }
    }

    impl Opts {
        pub fn lookup<U: magnus::TryConvert>(&self, key: &str) -> Result<Option<U>, Error> {
            self.opts_hash.lookup::<_, Option<U>>(magnus::Symbol::new(key))
        }

        pub fn lookup_str(&self, key: &str) -> Result<Option<String>, Error> {
            let value = self.lookup::<Value>(key)?;
            if let Some(object) = value {
                Ok(Some(unsafe { object.to_s() }?.into_owned()))
            } else {
                Ok(None)
            }
        }
    }
}
