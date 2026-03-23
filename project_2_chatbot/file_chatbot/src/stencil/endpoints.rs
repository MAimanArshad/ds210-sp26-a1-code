use std::sync::Arc;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use rocket::tokio::sync::Mutex;
use crate::stencil::adapter::Adapter;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    username: String,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    message: String,
}

#[post("/chat", format = "json", data = "<payload>")]
pub async fn endpoint(
    payload: Json<ChatRequest>,
    adapter: &State<Arc<Mutex<Adapter>>>,
) -> Json<ChatResponse> {
    let username = payload.username.clone();
    let message = payload.message.clone();
    let mut lock = adapter.lock().await;

    // Call solution function.
    let response_message = lock.call_solution(username, message).await;
    drop(lock);

    // Return the result.
    return Json(ChatResponse { message: response_message });
}

#[post("/history", format = "json", data = "<payload>")]
pub async fn history(
    payload: Json<ChatRequest>,
    adapter: &State<Arc<Mutex<Adapter>>>,
) -> Json<Vec<String>> {
    let username = payload.username.clone();
    let lock = adapter.lock().await;

    // Call solution function.
    let response_message = lock.get_history(username);
    drop(lock);

    // Return the result.
    return Json(response_message);
}