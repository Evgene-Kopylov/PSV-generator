use std::error::Error;

use teloxide::types::Message;

pub type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;
pub type HandlerMessage = Result<Message, Box<dyn Error + Send + Sync>>;
