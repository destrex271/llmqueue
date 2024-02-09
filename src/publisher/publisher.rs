use pgmq::{PgmqError, Message, PGMQueue};
use serde::{Serialize, Deserialize};
use crate::common::common::{QuestionMessage, ErrorMessage, AnswerMessage};
use chrono::{Utc};

const db_uri: &str = "postgres://postgres:postgres@localhost:5432";

pub async fn send_question_message(msg: String, queue_name: &str) -> Result<usize, ErrorMessage>{
    // Prepare question struct
    let prompt = QuestionMessage{
        question: msg,
        timestamp: Utc::now().to_string()
    };

    // Access Queue
    let queue: PGMQueue = PGMQueue::new(db_uri.to_string()).await.unwrap();

    // match queue{
    //     PgmqError => return Err(ErrorMessage{msg: "Unable to Connect to Postgres".to_string(), code: 100 as i8}),
    //     _ => println!("OK")
    // };
    
    let _msg_id = queue.send(&queue_name, &prompt)
        .await
        .expect("Unable to send message to queue");
    // println!("Sent to queue, {:?}", prompt);

    Ok(200 as usize)

}


pub async fn send_answer_message(msg: String, queue_name: String) -> Result<(), ErrorMessage>{
    // Prepare question struct
    let prompt = AnswerMessage{
        prompt_reply: msg,
        timestamp: Utc::now().to_string()
    };

    // Access Queue
    let queue: PGMQueue = PGMQueue::new(db_uri.to_string())
        .await
        .expect("Failed to connect to postgres");
    
    let _msg_id = queue.send(&queue_name, &prompt)
        .await
        .expect("Unable to send message to queue");
    println!("Sent to queue, {:?}", prompt);

    Ok(())

}
