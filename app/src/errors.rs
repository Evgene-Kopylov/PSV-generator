use std::error::Error;

use teloxide::types::Message;

pub type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;
pub type ResultError = Box<dyn Error + Send + Sync>;
pub type HandlerMessageResult = Result<Message, Box<dyn Error + Send + Sync>>;
