use kalosm::language::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct ChatbotV3 {
    model: Llama,
    users: HashMap<String, Chat<Llama>>,
}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        let users: HashMap<String, Chat<Llama>> = HashMap::new();
        return ChatbotV3 {
            model,
            users,
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        if !self.users.contains_key(&username){
            let chat_session: Chat<Llama> = self.model.chat();
            self.users.insert(username.clone(), chat_session);
        }
        let output = self.users.get_mut(&username).unwrap().add_message(message).await;
        match output {
            Ok(response) => response,
            Err(e) => format!("Error: {}", e),
        }
    }

    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
        if let Some(chat) = self.users.get(&username){
            let session = chat.session().unwrap();
            let history = session.history();
            let mut result = Vec::new();
            for msg in history {
                result.push(msg.content().to_string());
            }
            return result;
        }
        else{
            return Vec::new();
        }
    }
}