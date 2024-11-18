use std::env;
use x_ai::client::XaiClient;
use x_ai::get_mod::ModelRequestBuilder;
use x_ai::traits::{ClientConfig, ModelInfoFetcher};

#[tokio::test]
async fn test_fetch_model_info() {
    let client = XaiClient::builder()
        .build()
        .expect("Failed to build XaiClient");

    client.set_api_key(
        env::var("XAI_API_KEY")
            .expect("XAI_API_KEY must be set!")
            .to_string(),
    );

    let model_id = "grok-beta".to_string();
    let request_builder = ModelRequestBuilder::new(client, model_id);

    let result = request_builder.fetch_model_info().await;

    assert!(result.is_ok());

    let model_info = result.unwrap();
    assert_eq!(model_info.id, "grok-beta");
    assert_eq!(model_info.object, "model");
    assert_eq!(model_info.owned_by, "xai");
    assert_eq!(model_info.created, 1727136000);
}
