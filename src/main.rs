mod publisher;
mod common;
mod consumer;
mod embeddings;

use pgmq::PgmqError;
use common::common::init_queue;
use publisher::publisher::send_question_message;
use consumer::consumer::consume_question_msg;

#[tokio::main(flavor="current_thread")]
async fn main() -> Result<(), PgmqError> {
    let question_queue = "question_queue";
    // Initialize Question Queue
    init_queue(question_queue).await;

    // Send Question
    let question = send_question_message("What is the Earth?".to_string(), &question_queue).await;

    match question{
        Err(_) => println!("Error"),
        Ok(_) => println!("Done")
    };

    // Receive Question
    let msg = consume_question_msg(&question_queue).await.unwrap();
    println!("Message is : {}", msg);

    Ok(())
}
