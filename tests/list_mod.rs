use std::env;
use x_ai::client::XaiClient;
use x_ai::list_mod::ReducedModelListRequestBuilder;
use x_ai::traits::ClientConfig;
use x_ai::traits::ListModelFetcher;

#[tokio::test]
async fn test_fetch_reduced_model_list() {
    let client = XaiClient::builder()
        .build()
        .expect("Failed to build XaiClient");

    client.set_api_key(
        env::var("XAI_API_KEY")
            .expect("XAI_API_KEY must be set!")
            .to_string(),
    );

    let request_builder = ReducedModelListRequestBuilder::new(client);

    let result = request_builder.fetch_model_info().await;

    assert!(result.is_ok());

    let model_list = result.unwrap();
    assert!(!model_list.data.is_empty());

    let first_model = &model_list.data[0];
    assert_eq!(first_model.id, "grok-beta");
    assert_eq!(first_model.object, "model");
    assert_eq!(first_model.owned_by, "xai");
    assert_eq!(first_model.created, 1727136000);

    let second_model = &model_list.data[1];
    assert_eq!(second_model.id, "grok-vision-beta");
    assert_eq!(second_model.object, "model");
    assert_eq!(second_model.owned_by, "xai");
    assert_eq!(second_model.created, 1730764800);
}
