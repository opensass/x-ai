use x_ai::chat_compl::ChatCompletionsRequestBuilder;
use x_ai::client::XaiClient;
use x_ai::traits::ChatCompletionsFetcher;

use std::env;
use x_ai::chat_compl::Message;
use x_ai::traits::ClientConfig;

#[tokio::test]
async fn test_chat_completion() {
    let client = XaiClient::builder()
        .build()
        .expect("Failed to build XaiClient");

    client.set_api_key(
        env::var("XAI_API_KEY")
            .expect("XAI_API_KEY must be set!")
            .to_string(),
    );

    let messages = vec![
        Message {
            role: "system".to_string(),
            content: "You are Grok, a chatbot inspired by the Hitchhikers Guide to the Galaxy."
                .to_string(),
        },
        Message {
            role: "user".to_string(),
            content: "What is the answer to life and universe?".to_string(),
        },
    ];

    let request_builder =
        ChatCompletionsRequestBuilder::new(client.clone(), "grok-beta".to_string(), messages)
            .temperature(0.0)
            .stream(false);

    let request = request_builder
        .clone()
        .build()
        .expect("Failed to build request");
    println!(
        "Request payload: {:?}",
        serde_json::to_string(&request).unwrap()
    );

    let response = request_builder.create_chat_completion(request).await;
    assert!(response.is_ok(), "Request failed: {:?}", response.err());

    let completion = response.unwrap();
    assert_eq!(completion.object, "chat.completion");
    assert!(completion.choices.len() > 0);
    assert_eq!(completion.choices[0].message.role, "assistant");
}
