use mockito::{Matcher, Server};
use x_ai::api_key::ApiKeyRequestBuilder;
use x_ai::client::XaiClient;
use x_ai::traits::ApiKeyFetcher;
use x_ai::traits::ClientConfig;

#[tokio::test]
async fn test_fetch_api_key_info() {
    let mut server = Server::new_async().await;

    let mock_response = r#"
    {
        "acls": ["api-key:model:*", "api-key:endpoint:*"],
        "api_key_blocked": false,
        "api_key_disabled": false,
        "api_key_id": "ae1e1841-4326-a8a9-8a1a7237db11",
        "create_time": "2024-01-01T12:55:18.139305Z",
        "modified_by": "3d38b4dc-4eb7-4785-ae26-c3fa8997ffc7",
        "modify_time": "2024-08-28T17:20:12.343321Z",
        "name": "My API Key",
        "redacted_api_key": "xG1k...b14o",
        "team_blocked": false,
        "team_id": "5ea6f6bd-7815-4b8a-9135-28b2d7ba6722",
        "user_id": "59fbe5f2-040b-46d5-8325-868bb8f23eb2"
    }
    "#;

    let _mock = server
        .mock("GET", "/api-key")
        .match_header("Authorization", Matcher::Regex(r"^Bearer .*".to_string()))
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(mock_response)
        .create_async()
        .await;

    let client = XaiClient::builder()
        .base_url(&server.url())
        .build()
        .expect("Failed to build XaiClient");

    client.set_api_key("test-api-key".to_string());

    let request_builder = ApiKeyRequestBuilder::new(client);

    let result = request_builder.fetch_api_key_info().await;

    assert!(result.is_ok());

    let api_key_info = result.unwrap();
    assert_eq!(api_key_info.api_key_id, "ae1e1841-4326-a8a9-8a1a7237db11");
    assert_eq!(api_key_info.name, "My API Key");
    assert!(!api_key_info.api_key_blocked);
    assert_eq!(
        api_key_info.acls,
        vec!["api-key:model:*", "api-key:endpoint:*"]
    );

    server.reset();
}
