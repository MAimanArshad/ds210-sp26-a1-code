use kalosm::language::*;
use crate::solution::file_library;

pub struct ChatbotV4 {
    model: Llama,
}

impl ChatbotV4 {
    pub fn new(model: Llama) -> ChatbotV4 {
        return ChatbotV4 {
            model: model,
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
    let filename = &format!("{}.txt", username);

    let mut chat_session: Chat<Llama> = self.model
        .chat()
        .with_system_prompt("The assistant will act like a pirate");

    let history = file_library::load_chat_session_from_file(filename);
    match history{
        Some(h) => {
            let mut continued_session = chat_session.with_session(h);
            let output = continued_session.add_message(message).await;
            match output{
                Ok(response) => {
                    file_library::save_chat_session_to_file(filename,&continued_session.session().unwrap());
                    return response;
                },
                Err(e) => format!("Error: {}", e),
            }
        }
        None =>{
            let output = chat_session.add_message(message).await;
            match output{
                Ok(response) => {
                    file_library::save_chat_session_to_file(filename,&chat_session.session().unwrap());
                    return response;
                },
                Err(e) => format!("Error: {}", e),
            }
        }
    }
}




    pub fn get_history(&self, username: String) -> Vec<String> {
    let filename = &format!("{}.txt", username);

    match file_library::load_chat_session_from_file(&filename) {
        None => {
            return Vec::new();
        },
        Some(session) => {
            return session
                .history()
                .iter()
                .filter(|msg| {
                    matches!(msg.role(), MessageType::UserMessage | MessageType::ModelAnswer)
                })
                .map(|msg| msg.content().to_string())
                .collect();
        }
    }
}
}