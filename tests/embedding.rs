use mockito::{Matcher, Server};
use reqwest::Method;
use serde_json::json;
use x_ai::client::XaiClient;
use x_ai::traits::ClientConfig;

#[tokio::test]
async fn test_create_embedding() {
    let mut server = Server::new_async().await;

    let mock_response = r#"
    {
        "data": [
            {
                "embedding": {
                    "Float": [0.01567895, 0.063257694, 0.045925662]
                },
                "index": 0,
                "object": "embedding"
            }
        ],
        "model": "v1",
        "object": "list"
    }
    "#;

    let embedding_mock = server
        .mock("POST", "/v1/embeddings")
        .match_header("Content-Type", "application/json")
        .match_body(Matcher::JsonString(
            r#"
            {
                "input": ["This is an example content to embed..."],
                "model": "v1",
                "encoding_format": "float"
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
        "input": ["This is an example content to embed..."],
        "model": "v1",
        "encoding_format": "float"
    });

    let result = client
        .request(Method::POST, "/v1/embeddings")
        .expect("Failed to create request")
        .json(&body)
        .send()
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status(), 200);

    let response_text = response.text().await.unwrap();
    assert_eq!(response_text, mock_response);

    embedding_mock.assert_async().await;
}
