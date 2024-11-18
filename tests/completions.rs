use std::env;
use x_ai::client::XaiClient;
use x_ai::completions::CompletionsRequestBuilder;
use x_ai::traits::{ClientConfig, CompletionsFetcher};
use x_ai::XAI_V1_URL;

#[tokio::test]
async fn test_create_completions() {
    let client = XaiClient::builder()
        .base_url(XAI_V1_URL)
        .build()
        .expect("Failed to build XaiClient");

    client.set_api_key(
        env::var("XAI_API_KEY")
            .expect("XAI_API_KEY must be set!")
            .to_string(),
    );

    let request_builder = CompletionsRequestBuilder::new(
        client.clone(),
        "grok-beta".to_string(),
        "What is AI?".to_string(),
    )
    .max_tokens(50)
    .temperature(0.5)
    .n(1)
    .stop(vec!["\n".to_string()]);

    let request = request_builder.clone().build();
    assert!(
        request.is_ok(),
        "Request builder failed: {:?}",
        request.err()
    );

    let response = request_builder
        .create_completions(request.expect("REASON"))
        .await;
    assert!(response.is_ok(), "Request failed: {:?}", response.err());

    let completions = response.unwrap();

    assert_eq!(completions.object, "text_completion");
    assert_eq!(completions.model, "grok-beta");
    assert!(completions.choices.len() > 0, "No choices returned");

    let choice = &completions.choices[0];
    assert!(choice.text.len() > 0, "Choice text is empty");
    assert!(
        matches!(choice.finish_reason.as_str(), "length" | "stop" | "null"),
        "Unexpected finish_reason"
    );

    if let Some(usage) = &completions.usage {
        assert!(
            usage.prompt_tokens > 0,
            "Prompt tokens should be greater than 0"
        );
        assert!(
            usage.completion_tokens > 0,
            "Completion tokens should be greater than 0"
        );
        assert_eq!(
            usage.total_tokens,
            usage.prompt_tokens + usage.completion_tokens,
            "Token count mismatch"
        );
    }
}
