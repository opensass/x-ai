use std::env;
use x_ai::client::XaiClient;
use x_ai::list_lang_mod::LanguageModelRequestBuilder;
use x_ai::traits::ClientConfig;
use x_ai::traits::ModelFetcher;

#[tokio::test]
async fn test_fetch_language_models() {
    let client = XaiClient::builder()
        .build()
        .expect("Failed to build XaiClient");

    client.set_api_key(
        env::var("XAI_API_KEY")
            .expect("XAI_API_KEY must be set!")
            .to_string(),
    );

    let request_builder = LanguageModelRequestBuilder::new(client);

    let result = request_builder.fetch_model_info().await;

    assert!(result.is_ok());

    let model_list = result.unwrap();
    assert!(!model_list.models.is_empty());

    let first_model = &model_list.models[0];
    assert_eq!(first_model.id, "grok-beta");
    assert_eq!(first_model.object, "model");
    assert_eq!(first_model.owned_by, "xai");
    assert!(first_model.input_modalities.contains(&"text".to_string()));
    assert!(first_model.output_modalities.contains(&"text".to_string()));
    assert_eq!(first_model.completion_text_token_price, 150000);
    assert_eq!(first_model.prompt_text_token_price, 50000);
    assert_eq!(first_model.prompt_image_token_price, 0);
}
