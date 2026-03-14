use kalosm::language::Llama;

#[cfg(feature = "v1")]
use crate::solution::v1::ChatbotV1;

#[cfg(feature = "v2")]
use crate::solution::v2::ChatbotV2;

#[cfg(feature = "v3")]
use crate::solution::v3::ChatbotV3;

pub struct Adapter {
    #[cfg(feature = "v1")]
    chatbot: ChatbotV1,
    #[cfg(feature = "v2")]
    chatbot: ChatbotV2,
    #[cfg(feature = "v3")]
    chatbot: ChatbotV3,
}

impl Adapter {
    pub fn new(_model: Llama) -> Adapter {
        #[cfg(not(any(feature = "v1", feature = "v2", feature = "v3")))]
        panic!("You did not select which solution to run!");

        #[cfg(any(feature = "v1", feature = "v2", feature = "v3"))]
        return Adapter {
            #[cfg(feature = "v1")]
            chatbot: ChatbotV1::new(_model),
            #[cfg(feature = "v2")]
            chatbot: ChatbotV2::new(_model),
            #[cfg(feature = "v3")]
            chatbot: ChatbotV3::new(_model),
        };
    }

    pub fn get_history(&self, username: String) -> Vec<String> {
        println!("{username} logged in!");

        #[cfg(feature = "v3")]
        return self.chatbot.get_history(username);

        #[cfg(not(feature = "v3"))]
        return Vec::new();
    }

    pub async fn call_solution(&mut self, username: String, message: String) -> String {
        println!("Received message {message} from {username}");
        println!("Working on coming up with a response (will take some time)..");

        #[cfg(feature = "v1")]
        return self.chatbot.chat_with_user(message).await;
        #[cfg(feature = "v2")]
        return self.chatbot.chat_with_user(message).await;
        #[cfg(feature = "v3")]
        return self.chatbot.chat_with_user(username, message).await;

        #[cfg(not(any(feature = "v1", feature = "v2", feature = "v3")))]
        panic!("You did not select which solution to run!");
    }
}