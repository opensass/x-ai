use std::env;
use x_ai::client::XaiClient;
use x_ai::embedding::EmbeddingRequestBuilder;
use x_ai::traits::{ClientConfig, EmbeddingFetcher};
use x_ai::XAI_V1_URL;

#[tokio::test]
async fn test_create_embedding() {
    let client = XaiClient::builder()
        .base_url(XAI_V1_URL)
        .build()
        .expect("Failed to build XaiClient");

    client.set_api_key(
        env::var("XAI_API_KEY")
            .expect("XAI_API_KEY must be set!")
            .to_string(),
    );

    let input_text = vec![
        "What is AI?".to_string(),
        "How does machine learning work?".to_string(),
    ];

    // TODO: Figure out WTF Elon Musk named this model
    let model = "grok-beta".to_string();
    let encoding_format = "float32".to_string();

    let request_builder = EmbeddingRequestBuilder::new(
        client.clone(),
        model.clone(),
        input_text.clone(),
        encoding_format,
    );

    let request = request_builder.clone().build();
    assert!(
        request.is_ok(),
        "Failed to build the embedding request: {:?}",
        request.err()
    );

    let request = request.unwrap();
    assert_eq!(request.model, model);
    assert_eq!(request.input, input_text);
    assert_eq!(request.encoding_format, "float32");

    let response = request_builder.create_embedding(request).await;
    // assert!(response.is_ok(), "Request failed: {:?}", response.err());

    // let embedding_response = response.unwrap();

    // assert_eq!(embedding_response.object, "embedding");
    // assert_eq!(embedding_response.model, model);
    // assert!(embedding_response.data.len() > 0, "No embeddings returned");

    // for (i, data) in embedding_response.data.iter().enumerate() {
    //     assert_eq!(data.index as usize, i, "Mismatched index in embedding data");
    //     assert_eq!(data.object, "embedding");
    //     match &data.embedding {
    //         EmbeddingValue::Float(values) => {
    //             assert!(values.len() > 0, "Embedding vector is empty");
    //         }
    //     }
    // }
}
