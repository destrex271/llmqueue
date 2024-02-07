use pgmq::{PgmqError, Message, PGMQueue};
use serde::{Serialize, Deserialize};
use crate::message::message::{QuestionMessage, ErrorMessage};
use chrono::{Utc};

const db_uri: String = "postgres://postgres:postgres@localhost:5432".to_string();
const queue_name: String = "question_queue".to_string();

async fn send_message(msg: String) -> Result<(), ErrorMessage>{
    // Prepare question struct
    //
    let prompt = QuestionMessage{
        question: msg,
        timestamp: Utc::now().to_string()
    };


    // Access Queue
    let queue: PGMQueue = PGMQueue::new(db_uri)
        .await
        .expect("Failed to connect to postgres");
    

    queue.create(&queue_name)
        .await
        .expect("Unable to create a questions queue");

    let _msg_id = queue.send(&queue_name, &prompt)
        .await
        .expect("Unable to send message to queue");

    Ok(())

}
