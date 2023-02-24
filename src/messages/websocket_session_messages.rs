use actix::{Message};

pub struct TextMessage {
    pub message: String,
}
impl Message for TextMessage {
    type Result = ();
}

