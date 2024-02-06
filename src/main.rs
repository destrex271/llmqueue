use pgmq::{PgmqError, Message, PGMQueue};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[tokio::main(flavor="current_thread")]
async fn main() -> Result<(), PgmqError> {
    // Connect to postgres
    println!("Connecting to PostgreSQL...");
    let db_uri: &str = "postgres://postgres:postgres@localhost:5432";
    let queue: PGMQueue = PGMQueue::new(db_uri.to_owned())
        .await
        .expect("Failed to connect to PostgreSQL");

    // Create queue
    let my_queue = "llm_queue".to_owned();
    queue.create(&my_queue)
        .await
        .expect("Failed to create queue");
    // Send message
    #[derive(Serialize, Debug, Deserialize)]
    struct PgMessage{
        name: String
    }

    let msg = PgMessage{
        name: "Akshat Jaimini".to_string()
    };
    let msg_id = queue
        .send(&my_queue, &msg)
        .await
        .expect("Unable to send message");

    let visibility_timeout: i32 = 10;
    let received_msg: Message<PgMessage> = queue
        .read::<PgMessage>(&my_queue, Some(visibility_timeout))
        .await
        .unwrap()
        .expect("No message received in queue");

    println!("Message Received: {:?}", received_msg);
    Ok(())
}
