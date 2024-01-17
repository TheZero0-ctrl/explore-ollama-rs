use futures::StreamExt;
use tokio::io::AsyncWriteExt;
use ollama_rs::{
    Ollama,
    generation::{
        completion::{
            request::GenerationRequest,
            GenerationFinalResponseData,
        },
        chat::request::ChatMessageRequest,
    },
};

pub async fn gen_stream_print(
    ollama: &Ollama,
    generate_req: GenerationRequest,
) -> anyhow::Result<Option<GenerationFinalResponseData>> {
    let mut stream = ollama.generate_stream(generate_req).await.unwrap();
    let mut stdout = tokio::io::stdout();
    while let Some(res) = stream.next().await {
        let res = res.unwrap();
        stdout.write(res.response.as_bytes()).await.unwrap();
        stdout.flush().await.unwrap();

        if let Some(final_data) = res.final_data {
            stdout.write_all(b"\n").await.unwrap();
            stdout.flush().await.unwrap();
            return Ok(Some(final_data));
        }
    }
    Ok(None)
}

pub async fn run_chat_request(
    ollama: &Ollama,
    chat_req: ChatMessageRequest,
) -> anyhow::Result<Option<String>> {

    let mut stream = ollama.send_chat_messages_stream(chat_req).await?;
    let mut current_asst_msg: Vec<String> = Vec::new();
    let mut stdout = tokio::io::stdout();

    while let Some(res) = stream.next().await {
        let res = res.unwrap();

        if let Some(msg) = res.message {
           let msg_content = msg.content;
            stdout.write(msg_content.as_bytes()).await.unwrap();
            stdout.flush().await.unwrap();
            current_asst_msg.push(msg_content);
        }

        if let Some(_final_res) = res.final_data {
            stdout.write_all(b"\n").await.unwrap();
            stdout.flush().await.unwrap();

            let asst_content = current_asst_msg.join("");
            
            return Ok(Some(asst_content));
        }
    }

    stdout.write_all(b"\n").await.unwrap();
    stdout.flush().await.unwrap();

    Ok(None)
}

