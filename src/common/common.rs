// Created on 6th February, 2023 - by Akshat Jaimini
// Just contains data structures for messages etc..
//

use pgmq::{PgmqError, Message, PGMQueue};
use serde::{Serialize, Deserialize};

const db_uri: &str = "postgres://postgres:postgres@localhost:5432";

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionMessage{
    pub question: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnswerMessage{
    pub prompt_reply: String,
    pub timestamp: String
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessage{
    pub msg: String,
    pub code: i8
}

pub async fn init_queue(queue_name: &str){
    let queue: PGMQueue = PGMQueue::new(db_uri.to_string())
        .await
        .expect("Failed to connect to postgres");

    queue.create(&queue_name)
            .await
            .expect("Unable to create a questions queue");
    println!("Created Queue!");
}

