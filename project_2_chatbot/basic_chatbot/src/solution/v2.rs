use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV2 {
    model: Llama,
    chat_session: Chat<Llama>,
}

impl ChatbotV2 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV2 {
        let chat_session = model.chat();
        return ChatbotV2 {model, chat_session}
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        let output = self.chat_session.add_message(message).await;
        match output {
            Ok(response) => response,
            Err(e) => format!("Error: {}", e),
        }
    }
}