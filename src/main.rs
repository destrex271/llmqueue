mod publisher;
mod common;
mod consumer;
mod llm;

use pgmq::PgmqError;
use common::common::init_queue;
use publisher::publisher::send_question_message;
use consumer::consumer::consume_question_msg;
use llm::llama::{LlamaInstance, LlamaFunctions};

#[tokio::main(flavor="current_thread")]
async fn main() { //-> Result<(), PgmqError>{

    // Demo Send message to llm
    let llm: LlamaInstance = LlamaFunctions::new("llama2:latest".to_string(), "http://0.0.0.0".to_string(), 11434);
    let response = llm.generate_response("How are you Llama?".to_string()).await;

    if let Ok(response) = response {
        println!("{}", response);
    }
}
