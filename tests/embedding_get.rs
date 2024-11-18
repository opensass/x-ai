use std::env;
use x_ai::client::XaiClient;
use x_ai::embedding_get::EmbeddingModelRequestBuilder;
use x_ai::traits::ClientConfig;
use x_ai::traits::EmbeddingModelFetcher;

#[tokio::test]
async fn test_fetch_embedding_model_info() {
    let client = XaiClient::builder()
        .build()
        .expect("Failed to build XaiClient");

    client.set_api_key(
        env::var("XAI_API_KEY")
            .expect("XAI_API_KEY must be set!")
            .to_string(),
    );

    let model_id = "v1";

    let request_builder = EmbeddingModelRequestBuilder::new(client.clone(), model_id.to_string());

    let _result = request_builder.fetch_model_info().await;

    // TODO: Fix this after create_embedding endpoint
    // assert!(result.is_ok());

    // let model_info = result.unwrap();
    // assert_eq!(model_info.id, "v1");
    // assert_eq!(model_info.object, "embedding_model");
    // assert_eq!(model_info.owned_by, "xai");
    // assert_eq!(model_info.version, "1.0.0");
    // assert!(model_info.input_modalities.contains(&"text".to_string()));
    // assert!(model_info.input_modalities.contains(&"image".to_string()));
    // assert_eq!(model_info.prompt_text_token_price, 1);
    // assert_eq!(model_info.prompt_image_token_price, 2);
}
