use mockito::{Matcher, Server};
use reqwest::Method;
use serde_json::json;
use x_ai::client::XaiClient;
use x_ai::traits::ClientConfig;

#[tokio::test]
async fn test_chat_completions() {
    let mut server = Server::new_async().await;

    let chat_completion_mock = server
        .mock("POST", "/v1/chat/completions")
        .match_header("Content-Type", "application/json")
        .match_body(Matcher::JsonString(r#"
            {
                "messages": [
                    {
                        "role": "system",
                        "content": "You are Grok, a chatbot inspired by the Hitchhikers Guide to the Galaxy."
                    },
                    {
                        "role": "user",
                        "content": "What is the answer to life and universe?"
                    }
                ],
                "model": "grok-beta",
                "stream": false,
                "temperature": 0
            }
        "#.to_string()))
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(r#"
            {
              "id": "304e12ef-81f4-4e93-a41c-f5f57f6a2b56",
              "object": "chat.completion",
              "created": 1728511727,
              "model": "grok-beta",
              "choices": [
                {
                  "index": 0,
                  "message": {
                    "role": "assistant",
                    "content": "The answer to the ultimate question of life, the universe, and everything is **42**, according to Douglas Adams science fiction series \"The Hitchhiker's Guide to the Galaxy.\" This number is often humorously referenced in discussions about the meaning of life. However, in the context of the story, the actual question to which 42 is the answer remains unknown, symbolizing the ongoing search for understanding the purpose or meaning of existence."
                  },
                  "finish_reason": "stop"
                }
              ],
              "usage": {
                "prompt_tokens": 24,
                "completion_tokens": 91,
                "total_tokens": 115
              },
              "system_fingerprint": "fp_3813298403"
            }
        "#)
        .create_async()
        .await;

    let client = XaiClient::builder()
        .base_url(&format!("{}/", server.url()))
        .build()
        .expect("Failed to build XaiClient");

    client.set_api_key("test-api-key".to_string());

    let body = json!({
        "messages": [
            {
                "role": "system",
                "content": "You are Grok, a chatbot inspired by the Hitchhikers Guide to the Galaxy."
            },
            {
                "role": "user",
                "content": "What is the answer to life and universe?"
            }
        ],
        "model": "grok-beta",
        "stream": false,
        "temperature": 0
    });

    let result = client
        .request(Method::POST, "/v1/chat/completions")
        .expect("body")
        .json(&body)
        .send()
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status(), 200);

    let response_text = response.text().await.unwrap();
    assert_eq!(
        response_text,
        r#"
            {
              "id": "304e12ef-81f4-4e93-a41c-f5f57f6a2b56",
              "object": "chat.completion",
              "created": 1728511727,
              "model": "grok-beta",
              "choices": [
                {
                  "index": 0,
                  "message": {
                    "role": "assistant",
                    "content": "The answer to the ultimate question of life, the universe, and everything is **42**, according to Douglas Adams science fiction series \"The Hitchhiker's Guide to the Galaxy.\" This number is often humorously referenced in discussions about the meaning of life. However, in the context of the story, the actual question to which 42 is the answer remains unknown, symbolizing the ongoing search for understanding the purpose or meaning of existence."
                  },
                  "finish_reason": "stop"
                }
              ],
              "usage": {
                "prompt_tokens": 24,
                "completion_tokens": 91,
                "total_tokens": 115
              },
              "system_fingerprint": "fp_3813298403"
            }
        "#
    );

    chat_completion_mock.assert_async().await;
}
