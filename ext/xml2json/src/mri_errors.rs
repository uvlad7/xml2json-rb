pub type ExceptionClass = magnus::ExceptionClass;
pub type Error = magnus::Error;

pub fn type_error() -> ExceptionClass {
    magnus::exception::type_error()
}

pub fn runtime_error() -> ExceptionClass {
    magnus::exception::runtime_error()
}