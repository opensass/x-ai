use std::env;
use x_ai::api_key::ApiKeyRequestBuilder;
use x_ai::client::XaiClient;
use x_ai::traits::ApiKeyFetcher;
use x_ai::traits::ClientConfig;

#[tokio::test]
async fn test_fetch_api_key_info() {
    let client = XaiClient::builder()
        .build()
        .expect("Failed to build XaiClient");

    client.set_api_key(
        env::var("XAI_API_KEY")
            .expect("XAI_API_KEY must be set!")
            .to_string(),
    );

    let request_builder = ApiKeyRequestBuilder::new(client);

    let result = request_builder.fetch_api_key_info().await;

    assert!(result.is_ok());

    let api_key_info = result.unwrap();
    assert_eq!(
        api_key_info.api_key_id,
        "06e3dd66-267d-4bfb-b0db-ae23015e7f61"
    );
    assert_eq!(api_key_info.name, "Default");
    assert!(!api_key_info.api_key_blocked);
    assert_eq!(
        api_key_info.acls,
        vec!["api-key:model:*", "api-key:endpoint:*", ""]
    );
}
