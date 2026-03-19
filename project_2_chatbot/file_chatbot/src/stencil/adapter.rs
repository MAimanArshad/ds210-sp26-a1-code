use kalosm::language::Llama;

use crate::solution::v4::ChatbotV4;

pub struct Adapter {
    chatbot: ChatbotV4,
}

impl Adapter {
    pub fn new(model: Llama) -> Adapter {
        return Adapter {
            chatbot: ChatbotV4::new(model),
        };
    }

    pub fn get_history(&self, username: String) -> Vec<String> {
        return self.chatbot.get_history(username);
    }

    pub async fn call_solution(&mut self, username: String, message: String) -> String {
        println!("Received message {message} from {username}");
        println!("Working on coming up with a response (will take some time)..");
        return self.chatbot.chat_with_user(username, message).await;
    }
}