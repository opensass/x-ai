use std::env;
use x_ai::client::XaiClient;
use x_ai::traits::ClientConfig;
use x_ai::traits::EmbeddingModelsFetcher;

#[tokio::test]
async fn test_list_embedding_models_endpoint() {
    let client = XaiClient::builder()
        .build()
        .expect("Failed to build XaiClient");

    client.set_api_key(
        env::var("XAI_API_KEY")
            .expect("XAI_API_KEY must be set!")
            .to_string(),
    );

    let result = client.list_embedding_models().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.models.len(), 0);

    // TODO: Fix this after create_embedding endpoint
    // let model = &response.models[0];
    // assert_eq!(model.id, "v1");
    // assert_eq!(model.created, 1725148800);
    // assert_eq!(model.object, "model");
    // assert_eq!(model.owned_by, "xai");
    // assert_eq!(model.version, "0.1.0");
    // assert_eq!(model.input_modalities, vec!["text"]);
    // assert_eq!(model.prompt_image_token_price, 0);
    // assert_eq!(model.prompt_text_token_price, 100);
}
