use mockito::{Matcher, Server};
use reqwest::Method;
use serde_json::json;
use x_ai::client::XaiClient;
use x_ai::traits::ClientConfig;

#[tokio::test]
async fn test_completions_endpoint() {
    let mut server = Server::new_async().await;

    let mock_response = r#"
    {
        "choices": [],
        "created": 0,
        "id": "",
        "model": "",
        "object": "",
        "system_fingerprint": "",
        "usage": null
    }
    "#;

    let completions_mock = server
        .mock("POST", "/v1/completions")
        .match_header("Content-Type", "application/json")
        .match_body(Matcher::JsonString(
            r#"
            {
                "model": "grok-beta",
                "prompt": "What is the meaning of life?",
                "best_of": 1,
                "echo": false,
                "max_tokens": 100,
                "temperature": 0.7,
                "n": 1,
                "top_p": 1
            }
        "#
            .to_string(),
        ))
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(mock_response)
        .create_async()
        .await;

    let client = XaiClient::builder()
        .base_url(&format!("{}/", server.url()))
        .build()
        .expect("Failed to build XaiClient");

    client.set_api_key("test-api-key".to_string());

    let body = json!({
        "model": "grok-beta",
        "prompt": "What is the meaning of life?",
        "best_of": 1,
        "echo": false,
        "max_tokens": 100,
        "temperature": 0.7,
        "n": 1,
        "top_p": 1
    });

    let result = client
        .request(Method::POST, "/v1/completions")
        .expect("body")
        .json(&body)
        .send()
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status(), 200);

    let response_text = response.text().await.unwrap();
    assert_eq!(response_text, mock_response);

    completions_mock.assert_async().await;
}
