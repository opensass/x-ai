<div align="center">

# ✨ 𝕏-AI

[![CircleCI](https://dl.circleci.com/status-badge/img/circleci/2rCDyKJRUEePhb1wtzWHqR/wCPbiPVjGAET17HXk9U2d/tree/main.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/circleci/2rCDyKJRUEePhb1wtzWHqR/wCPbiPVjGAET17HXk9U2d/tree/main)
![Version](https://img.shields.io/crates/v/x_ai)
![Downloads](https://img.shields.io/crates/d/x_ai)
![License](https://img.shields.io/crates/l/x_ai)
![Last Commit](https://img.shields.io/github/last-commit/opensass/x-ai)
![Docs](https://img.shields.io/docsrs/x-ai)

![banner](https://github.com/user-attachments/assets/7b420bc7-fcfe-4aa6-b79a-6dfe1a2c019b)

</div>

> **𝕏-AI** is a **complete SDK** and a WIP **CLI/TUI** that provides a powerful and intuitive interface to interact with the [**X-AI API**](https://docs.x.ai/api/).

---

## 📜 Table of Contents

1. [Features](#-features)
2. [Installation](#-installation)
3. [Usage Examples](#-usage-examples)
   - [Fetch API Key Information 🔑](#fetch-api-key-information-)
   - [Chat Completions 💬](#chat-completions-)
   - [Text Completions 📝](#text-completions-)
   - [Embedding Creation 📊](#embedding-creation-)
   - [List Models 📜](#list-models-)
4. [Examples](#-examples)
5. [Contributing](#-contributing)
6. [License](#-license)

---

## ✨ Features

- Fetch API Key Information 🔑
- Chat Completions 💬
- Text Completions 📝
- Embedding Creation 📊
- Fetch Model Information 🧐
- List Embedding Models 📜
- Fetch Language Model Details 🌐
- List Language Models 🗃️

---

## 📦 Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
x_ai = "0.0.1"
tokio = { version = "1.41.1", features = ["full"] }
```

---

## 🛠️ Usage Examples

### Fetch API Key Information 🔑

```rust
use std::env;
use x_ai::api_key::ApiKeyRequestBuilder;
use x_ai::client::XaiClient;
use x_ai::traits::ApiKeyFetcher;
use x_ai::traits::ClientConfig;

#[tokio::main]
async fn main() {
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

    match result {
        Ok(api_key_info) => println!("API Key ID: {}", api_key_info.api_key_id),
        Err(err) => eprintln!("Error fetching API key info: {:?}", err),
    }
}

// Output

// API Key ID: 06e3dd6...5e7f61
```

---

### Chat Completions 💬

```rust
use std::env;
use x_ai::chat_compl::ChatCompletionsRequestBuilder;
use x_ai::client::XaiClient;
use x_ai::traits::ChatCompletionsFetcher;
use x_ai::chat_compl::Message;
use x_ai::traits::ClientConfig;

#[tokio::main]
async fn main() {
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
            content: "You are Grok, a chatbot inspired by the Hitchhiker's Guide to the Galaxy."
                .to_string(),
        },
        Message {
            role: "user".to_string(),
            content: "What is the answer to life and the universe?".to_string(),
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

    let response = request_builder.create_chat_completion(request).await;
    match response {
        Ok(completion) => {
            println!("Chatbot Response: {}", completion.choices[0].message.content);
        }
        Err(err) => eprintln!("Error: {:?}", err),
    }
}

// Output

// Chatbot Response: The answer to life, the universe, and everything is **42**. However, this answer is famously incomplete without knowing the question, which remains unknown. This concept comes from Douglas Adams' "The Hitchhiker's Guide to the Galaxy." If you're looking for deeper meaning or a more personal answer, I'd say it's about finding what gives your life purpose and joy, which can be quite different for everyone. What do you think might be your personal answer to life and the universe?
```

---

### Text Completions 📝

```rust
use std::env;
use x_ai::client::XaiClient;
use x_ai::completions::CompletionsRequestBuilder;
use x_ai::traits::{ClientConfig, CompletionsFetcher};

#[tokio::main]
async fn main() {
    let client = XaiClient::builder()
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
        "Write a short poem about Rust programming.".to_string(),
    )
    .temperature(0.7)
    .max_tokens(50);

    let request = request_builder.clone()
        .build()
        .expect("Failed to build request");

    let response = request_builder.create_completions(request).await;

    match response {
        Ok(completion) => println!("Generated Text: {}", completion.choices[0].text),
        Err(err) => eprintln!("Error: {:?}", err),
    }
}

// Output

// Generated Text:  Make the poem rhyme.

// In the land of code, a language so bright,
// Rust emerges with all its might.
// With safety and speed, it's quite the sight,
// Guarding memory with all its might.

// Fear not the bugs, nor the seg
```

---

### Embedding Creation 📊

```rust
use std::env;
use x_ai::client::XaiClient;
use x_ai::embedding::EmbeddingRequestBuilder;
use x_ai::traits::ClientConfig;
use x_ai::traits::EmbeddingFetcher;

#[tokio::main]
async fn main() {
    let client = XaiClient::builder()
        .build()
        .expect("Failed to build XaiClient");

    client.set_api_key(
        env::var("XAI_API_KEY")
            .expect("XAI_API_KEY must be set!")
            .to_string(),
    );

    let input_texts = vec!["Hello, world!".to_string(), "Rust is awesome!".to_string()];
    let model = "text-embedding-3-small".to_string();
    let encoding_format = "float32".to_string();

    let request_builder =
        EmbeddingRequestBuilder::new(client.clone(), model, input_texts, encoding_format);

    let request = request_builder
        .clone()
        .build()
        .expect("Failed to build request");

    let response = request_builder.create_embedding(request).await;

    match response {
        Ok(embedding) => println!("Embedding Data: {:?}", embedding.data),
        Err(err) => eprintln!("Error: {:?}", err),
    }
}

// Output

// TODO
```

---

### List Models 📜

```rust
use std::env;
use x_ai::client::XaiClient;
use x_ai::list_mod::ReducedModelListRequestBuilder;
use x_ai::traits::ClientConfig;
use x_ai::traits::ListModelFetcher;

#[tokio::main]
async fn main() {
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

    match result {
        Ok(model_list) => {
            for model in model_list.data {
                println!("Model ID: {}, Owned By: {}", model.id, model.owned_by);
            }
        }
        Err(err) => eprintln!("Error fetching models: {:?}", err),
    }
}

// Output

// Model ID: grok-beta, Owned By: xai
// Model ID: grok-vision-beta, Owned By: xai
```

---

## 📌 Examples

> [!WARNING]  
> 🚧 Under Construction

This repository contains a list of notebooks examples on how to use the sdk and or the cli. To use the notebooks in this repository, you need to set up your environment. Follow these steps to get started:

1. Clone the repository to your local machine:

   ```sh
   git clone https://github.com/opensass/x-ai.git
   ```

1. Install the required dependencies and libraries. Make sure you have [`Rust`](https://rustup.rs/), [`Jupyter Notebook`](https://jupyter.org/install), and [`evcxr_jupyter`](https://github.com/evcxr/evcxr/blob/main/evcxr_jupyter/README.md) installed on your system.

   ```sh
   # Install a Rust toolchain (e.g. nightly):
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly

   # Although not recommended, but, You may need to clean the python thing cz of new v3.12 venv breaking changes
   sudo rm /usr/lib/python3.*/EXTERNALLY-MANAGED

   # Install Jupyter Notebook
   pip install notebook

   # Install evcxr_jupyter, upgrade soon
   cargo install evcxr_jupyter
   evcxr_jupyter --install
   ```

1. Navigate to the cloned repository and build the project:

   ```sh
   cd x-ai
   cargo build --release --all-features
   ```

1. Start Jupyter Notebook:

   ```sh
   jupyter notebook
   ```

1. Access the notebooks in your web browser by clicking on the notebook file you want to explore.

| ID | Example | Open on GitHub | Launch on Binder | Launch on Colab |
|----|---------------|-----------|:-------------|-------------|
| 1  | **Basic** | [![Github](https://img.shields.io/badge/launch-Github-181717.svg?logo=github&logoColor=white)](./examples/basic.ipynb) | [![Binder](https://mybinder.org/badge_logo.svg)](https://mybinder.org/v2/gh/opensass/x-ai/main?filepath=examples/basic.ipynb) |  [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/opensass/x-ai/blob/main/examples/basic.ipynb) |

---

## 🤝 Contributing

Contributions and feedback are welcome! If you'd like to contribute, report an issue, or suggest an enhancement, please engage with the project on [GitHub](https://github.com/opensass/x-ai). Your contributions help improve this crate for the community.

---

## 📜 License

This crate is licensed under the MIT License. See the [`LICENSE`](LICENSE) file for details.
