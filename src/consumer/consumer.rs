use pgmq::{PgmqError, Message, PGMQueue};
use serde::{Serialize, Deserialize};
use crate::common::common::{QuestionMessage, ErrorMessage, AnswerMessage};
use chrono::{Utc};

const db_uri: &str = "postgres://postgres:postgres@localhost:5432";

pub async fn consume_question_msg(queue_name: &str) -> Result<String, ErrorMessage>{
    let queue = PGMQueue::new(db_uri.to_string())
        .await
        .expect("Unable to connect to postgres");
    
    let recv_msg = queue
        .pop::<QuestionMessage>(&queue_name.to_string())
        .await
        .unwrap()
        .expect("Unable to receive Message");

    return Ok(recv_msg.message.question);
}


pub async fn consume_answer_msg(queue_name: String) -> Result<String, ErrorMessage>{
    let mut msg = String::new();

    let queue = PGMQueue::new(db_uri.to_string())
        .await
        .expect("Unable to connect to postgres");
    
    let recv_msg: Message<AnswerMessage> = queue
        .pop::<AnswerMessage>(&queue_name.to_string())
        .await
        .unwrap()
        .expect("Unable to read message");
    println!("Received Question: {:?}", recv_msg);

    msg = recv_msg.message.prompt_reply;
    Ok(msg)
}
