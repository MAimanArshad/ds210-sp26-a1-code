#[macro_use]
extern crate tokio;

extern crate experiments;

use std::fs;

#[tokio::main]
async fn main() {
    let _ = fs::remove_file("Sophie.txt");
    let _ = fs::remove_file("User1.txt");
    let _ = fs::remove_file("User2.txt");
    let _ = fs::remove_file("User3.txt");

    // V3
    let client = experiments::client_v3().await;

    // Send a message to v3.
    let (message, _) = experiments::send_chat_message(&client, "Sophie", "hello, I am Sophie!").await;

    // Retrieve history from v3.
    let (v3_messages, v3_time) = experiments::get_history(&client, "Sophie").await;
    assert_eq!(v3_messages.len(), 2);
    assert_eq!(v3_messages[0], "hello, I am Sophie!");
    assert_eq!(v3_messages[1], message);
    let _ = fs::remove_file("Sophie.txt");

    // V4
    let client = experiments::client_v4().await;

    // Send a message to v4.
    let (message, _) = experiments::send_chat_message(&client, "Sophie", "hello, I am Sophie!").await;

    // Retrieve history from v4.
    let (v4_messages, v4_time) = experiments::get_history(&client, "Sophie").await;
    assert_eq!(v4_messages.len(), 2);
    assert_eq!(v4_messages[0], "hello, I am Sophie!");
    assert_eq!(v4_messages[1], message);
    let _ = fs::remove_file("Sophie.txt");

    // V5
    let client = experiments::client_v5().await;

    // Send a message to v5.
    let (message, _) = experiments::send_chat_message(&client, "Sophie", "hello, I am Sophie!").await;

    // Retrieve history from v5 (hot).
    let (v5_messages, v5_time_hot) = experiments::get_history(&client, "Sophie").await;
    assert_eq!(v5_messages.len(), 2);
    assert_eq!(v5_messages[0], "hello, I am Sophie!");
    assert_eq!(v5_messages[1], message);

    // Send more message to v5 to make Sophie's conversation cold.
    let _ = experiments::send_chat_message(&client, "User1", "hello, I am user1!").await;
    let _ = experiments::send_chat_message(&client, "User2", "hello, I am user2!").await;
    let _ = experiments::send_chat_message(&client, "User3", "hello, I am user3!").await;
    let _ = fs::remove_file("User1.txt");
    let _ = fs::remove_file("User2.txt");
    let _ = fs::remove_file("User3.txt");

    // Retrieve history from v5 (cold).
    let (v5_messages, v5_time_cold) = experiments::get_history(&client, "Sophie").await;
    assert_eq!(v5_messages.len(), 2);
    assert_eq!(v5_messages[0], "hello, I am Sophie!");
    assert_eq!(v5_messages[1], message);
    let _ = fs::remove_file("Sophie.txt");

    println!("----- Experiment Results -----");
    println!(" v3:");
    println!("  retrieved history: {:?}", v3_messages);
    println!("  took: {:?}", v3_time);
    println!(" v4:");
    println!("  retrieved history: {:?}", v4_messages);
    println!("  took: {:?}", v4_time);
    println!(" v5:");
    println!("  retrieved history: {:?}", v5_messages);
    println!("  when Sophie is hot, it took: {:?}", v5_time_hot);
    println!("  when Sophie is cold, it took: {:?}", v5_time_cold);
    println!("----- End of Experiment Results -----");
}