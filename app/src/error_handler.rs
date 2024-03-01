use std::error::Error;

pub type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;
