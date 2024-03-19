mod publisher;
mod common;
mod consumer;
mod llm;

use pgmq::PgmqError;
use common::common::init_queue;
use publisher::publisher::send_question_message;
use consumer::consumer::consume_question_msg;
use llm::llama::{init_llama2};

#[tokio::main(flavor="current_thread")]
async fn main() { //-> Result<(), PgmqError>{

    println!("OK!");
    init_llama2().await;
}
