use ai::client::chat::{self, ChatCompletionClient};
use ai::openai::OpenAIClient;

#[tokio::test]
async fn chat_completion() {
    let api_key = match std::env::var("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("OPENAI_API_KEY not set, skipping test");
            return;
        }
    };

    let client = OpenAIClient::new();
    let req = chat::ChatCompletionRequest::new("gpt-4o-mini")
        .with_messages(vec![chat::ChatCompletionMessage::User {
            content: chat::Content::Text("Say hello in exactly 3 words.".to_string()),
            name: None,
        }])
        .with_max_completion_tokens(20);

    let res = match client.chat(&api_key, req).await {
        Ok(res) => res,
        Err(e) => {
            panic!("API error: {e}");
        }
    };

    println!("Response: {:#?}", res);
    assert!(!res.choices.is_empty(), "expected at least one choice");
    assert!(res.usage.is_some(), "expected usage data");

    let choice = &res.choices[0];

    if let chat::ChatCompletionMessage::Assistant { content, .. } = &choice.message {
        assert!(content.is_some(), "expected assistant content");
        println!("Assistant: {}", content.as_ref().unwrap());
    } else {
        panic!("expected assistant message, got {:?}", choice.message);
    }
}
