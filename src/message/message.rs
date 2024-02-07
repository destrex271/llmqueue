// Created on 6th February, 2023 - by Akshat Jaimini
// Just contains data structures for messages etc..
//

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct QuestionMessage{
    pub question: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize)]
pub struct AnswerMessage{
    pub prompt_reply: String,
    pub timestamp: String
}


#[derive(Serialize, Deserialize)]
pub struct ErrorMessage{
    pub msg: String,
    pub code: i8
}
