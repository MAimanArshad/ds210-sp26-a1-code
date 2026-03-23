use kalosm::language::*;
use file_chatbot::solution::file_library;

use crate::solution::Cache;

pub struct ChatbotV5 {
    model: Llama,
    cache: Cache<Chat<Llama>>,
}

impl ChatbotV5 {
    pub fn new(model: Llama) -> ChatbotV5 {
        return ChatbotV5 {
            model: model,
            cache: Cache::new(3),
        };
    }

pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
    let filename = format!("{}.txt", username);
    let cached_chat = self.cache.get_chat(&username);

    match cached_chat {
        None => {
            println!("chat_with_user: {username} is not in the cache!");
            //The cache does not have the chat. What should you do?
            let mut chat = match file_library::load_chat_session_from_file(&filename) {
                //File exists
                Some(session) => {
                    self.model
                        .chat()
                        .with_system_prompt("The assistant will act like a pirate")
                        .with_session(session)
                }
                //File doesn't exist
                None => {
                    self.model
                        .chat()
                        .with_system_prompt("The assistant will act like a pirate")
                }
            };
            let response = match chat.add_message(message).await {
                Ok(res) => res,
                Err(e) => return format!("Error: {}", e),
            };
            match chat.session() {
                Ok(session) => file_library::save_chat_session_to_file(&filename, &session),
                Err(e) => eprintln!("Failed to save session for {}: {}", username, e),
            }
            self.cache.insert_chat(username.clone(), chat);
            response
        }

        Some(chat) => {
            println!("chat_with_user: {username} is in the cache! Nice!");
            //The cache has this chat. What should you do?
            let response = match chat.add_message(message).await {
                Ok(res) => res,
                Err(e) => return format!("Error: {}", e),
            };
            match chat.session() {
                Ok(session) => file_library::save_chat_session_to_file(&filename, &session),
                Err(e) => eprintln!("Failed to save session for {}: {}", username, e),
            }
            response
        }
    }
}

    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
            println!("get_history: {username} is not in the cache!");
                match file_library::load_chat_session_from_file(filename) {
                    None => {
                        let chat = self.model.chat();
                        self.cache.insert_chat(username, chat);
                        return Vec::new();
                    },
                    Some(session) => {
                        let history = session
                            .history()
                            .iter()
                            .filter(|msg| matches!(msg.role(), MessageType::UserMessage | MessageType::ModelAnswer))
                            .map(|msg| msg.content().to_string())
                            .collect();
                        let chat = self.model.chat().with_session(session);
                        self.cache.insert_chat(username, chat);
                        return history;
                    }
                }
            }
            Some(chat_session) => {
                println!("get_history: {username} is in the cache! Nice!");
                match chat_session.session() {
                    Ok(session) => session
                        .history()
                        .iter()
                        .filter(|msg| matches!(msg.role(), MessageType::UserMessage | MessageType::ModelAnswer))
                        .map(|msg| msg.content().to_string())
                        .collect::<Vec<String>>(),
                    Err(r) => Vec::new(),
                }
            }
        }
    }
}