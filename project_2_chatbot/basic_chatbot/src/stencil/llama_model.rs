use kalosm::language::{Llama, LlamaSource};

// Downloads a Llama model if one has not been downloaded before and loads it.
pub async fn create_model() -> Llama {
    Llama::builder()
        .with_source(LlamaSource::llama_3_2_3b_chat())
        .build().await
        .unwrap()
}