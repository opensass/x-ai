use std::env;
use x_ai::client::XaiClient;
use x_ai::lang_mod::LanguageModelDetailRequestBuilder;
use x_ai::traits::ClientConfig;
use x_ai::traits::GetModelFetcher;

#[tokio::test]
async fn test_fetch_language_model_detail() {
    let client = XaiClient::builder()
        .build()
        .expect("Failed to build XaiClient");

    client.set_api_key(
        env::var("XAI_API_KEY")
            .expect("XAI_API_KEY must be set!")
            .to_string(),
    );

    let model_id = "grok-beta";

    let request_builder =
        LanguageModelDetailRequestBuilder::new(client.clone(), model_id.to_string());

    let result = request_builder.fetch_model_info().await;

    assert!(result.is_ok());

    let model_detail = result.unwrap();
    assert_eq!(model_detail.id, "grok-beta");
    assert_eq!(model_detail.object, "model");
    assert_eq!(model_detail.owned_by, "xai");
    assert!(model_detail.input_modalities.contains(&"text".to_string()));
    assert!(model_detail.output_modalities.contains(&"text".to_string()));
    assert_eq!(model_detail.completion_text_token_price, 150000);
    assert_eq!(model_detail.prompt_text_token_price, 50000);
    assert_eq!(model_detail.prompt_image_token_price, 0);
    assert_eq!(model_detail.version, "1.0.0");
}
