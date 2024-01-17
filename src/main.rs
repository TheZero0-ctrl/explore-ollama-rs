use anyhow::Ok;
use explore_ollama_rs::consts::{OLLAMA_MODEL, DEFAULT_SYSTEM_PROMPT};
use ollama_rs::{Ollama, generation::{completion::{request::GenerationRequest, GenerationContext}, chat::{ChatMessage, MessageRole, request::ChatMessageRequest}}};
use explore_ollama_rs::gen;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ollama = Ollama::default();
    // let model = OLLAMA_MODEL.to_string();
    // let prompt = "What is the second best programming language?".to_string();
    // let generate_req = GenerationRequest::new(model, prompt)
        // .system(DEFAULT_SYSTEM_PROMPT.to_string());
    // let res = ollama.generate(generate_req).await;
    // if let Ok(res) = res {
    //     println!("{}", res.response);
    // }
    // gen::gen_stream_print(&ollama, generate_req).await?;
    // with_context(&ollama).await?;
    chat(&ollama).await?;
    Ok(())
}

async fn with_context(ollama: &Ollama) -> anyhow::Result<()> {
    let prompts = &[
        "why the sky is red? (be concise)",
        "What was my first question?",
    ];

    let mut last_ctx: Option<GenerationContext> = None;

    for prompt in prompts {
        println!("->> Prompt: {}", prompt);
        let mut generate_req = GenerationRequest::new(
            OLLAMA_MODEL.to_string(),
            prompt.to_string(),
        );

        if let Some(ctx) = last_ctx.take() {
            generate_req = generate_req.context(ctx);
        }
        let final_data = gen::gen_stream_print(&ollama, generate_req).await?;
        
        if let Some(final_data) = final_data {
            last_ctx = Some(final_data.context);
        }
    }
    Ok(())
}

async fn chat(ollama: &Ollama) -> anyhow::Result<()> {
    let prompts = &[
        "What is the best programming language? (be concise)",
        "What is the second best programming language?",
        "What was my last question?",
    ];

    let system_message = ChatMessage::new(
        MessageRole::System,
        DEFAULT_SYSTEM_PROMPT.to_string(),
    );

    let mut chat_messages: Vec<ChatMessage> = vec![system_message];

    for prompt in prompts {
        println!("->> Prompt: {}", prompt);
        let prompt_message = ChatMessage::new(
            MessageRole::User,
            prompt.to_string(),
        );
        chat_messages.push(prompt_message);

        let chat_req = ChatMessageRequest::new(
            OLLAMA_MODEL.to_string(),
            chat_messages.clone(),
        );

        let message_content = gen::run_chat_request(ollama, chat_req).await?;

        if let Some(content) = message_content {
            let asst_message = ChatMessage::new(
                MessageRole::Assistant,
                content,
            );
            chat_messages.push(asst_message);
        }
    }
    Ok(())
}
