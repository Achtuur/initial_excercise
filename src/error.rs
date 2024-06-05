pub type Result<T> = core::result::Result<T, Error>;

pub type Error = Box<dyn std::error::Error>; // can convert str to error for some easy error handling
