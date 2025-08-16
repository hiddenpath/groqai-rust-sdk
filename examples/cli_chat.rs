// examples/cli_chat.rs
// CLI聊天应用示例，使用 llama-3.3-70b-versatile 模型
// 该模型提供强大的推理能力和高质量输出
use groqai::client::GroqClientBuilder;
use groqai::error::GroqError;
use groqai::types::{ChatMessage, Role, MessageContent};
use reqwest::Proxy;
use std::io::{self, Write};
use std::env;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), GroqError> {
    let proxy_url = env::var("PROXY_URL").expect("PROXY_URL must be set");
    println!("Using proxy: {}", proxy_url);
    let proxy = Proxy::all(&proxy_url).map_err(GroqError::from)?;

    let api_key = env::var("GROQ_API_KEY").expect("GROQ_API_KEY must be set");
    println!("API key: {} (redacted)", api_key.chars().take(4).collect::<String>());

    let client = GroqClientBuilder::new(api_key)
        .unwrap()
        .proxy(proxy)
        .build()?;
    println!("Base URL: {}", client.transport.base_url());

    let args: Vec<String> = env::args().collect();
    let stream = args.contains(&"--stream".to_string());

    let mut conversation_history = Vec::new();
    const MAX_HISTORY_PAIRS: usize = 15; // 保留最近15轮对话
    const MAX_TOKENS_ESTIMATE: usize = 18000; // 估算token限制

    loop {
        print!("Enter your message: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }

        let user_message = ChatMessage::new_text(Role::User, input.trim());
        conversation_history.push(user_message.clone());
        
        // 精简对话历史（但保证至少有当前消息）
        if conversation_history.len() > 1 {
            trim_conversation_history(&mut conversation_history, MAX_HISTORY_PAIRS, MAX_TOKENS_ESTIMATE);
        }
        
        // 在流式处理部分使用改进的错误处理
        if stream {
            let mut builder = client
                .chat("llama-3.3-70b-versatile")
                .temperature(0.7)
                .stream(true);
            
            for msg in &conversation_history {
                builder = builder.message(msg.clone());
            }
                
            let mut stream = match builder.send_stream().await {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to create stream: {:?}", e);
                    eprintln!("Message count: {}", conversation_history.len());
                    return Err(e);
                }
            };
            let mut ai_response = String::new();
            let mut consecutive_errors = 0;
            const MAX_CONSECUTIVE_ERRORS: u32 = 3;
            
            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(chunk) => {
                        consecutive_errors = 0;
                        
                        if let Some(choice) = chunk.choices.first() {
                            if let Some(content) = &choice.delta.content {
                                match content {
                                    groqai::types::MessageContent::Text(text) => {
                                        print!("{}", text);
                                        ai_response.push_str(text);
                                        io::stdout().flush().unwrap();
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }
                    Err(e) => {
                        consecutive_errors += 1;
                        eprintln!("Stream error ({}): {:?}", consecutive_errors, e);
                        
                        if consecutive_errors >= MAX_CONSECUTIVE_ERRORS {
                            eprintln!("Too many consecutive errors, stopping stream");
                            break;
                        }
                        
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    }
                }
            }
            println!();
            
            if !ai_response.is_empty() {
                conversation_history.push(ChatMessage::new_text(Role::Assistant, ai_response));
            }
        } else {
            let mut builder = client
                .chat("llama-3.1-8b-instant")
                .temperature(0.7)
                .stream(false);
            
            for msg in &conversation_history {
                builder = builder.message(msg.clone());
            }
                
            let response = match builder.send().await {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Failed to send request: {:?}", e);
                    eprintln!("Message count: {}", conversation_history.len());
                    return Err(e);
                }
            };
            if let Some(choice) = response.choices.first() {
                match &choice.message.content {
                    groqai::types::MessageContent::Text(text) => {
                        println!("Response: {}", text);
                        conversation_history.push(ChatMessage::new_text(Role::Assistant, text.clone()));
                    },
                    _ => println!("Unexpected response format"),
                }
            }
        }
        println!("--------------------------------\n");
    }
    Ok(())
}

// 精简对话历史的函数
fn trim_conversation_history(history: &mut Vec<ChatMessage>, max_pairs: usize, max_tokens: usize) {
    // 策略1: 滑动窗口 - 保留最近的对话轮次
    if history.len() > max_pairs * 2 {
        let keep_count = max_pairs * 2;
        history.drain(0..history.len() - keep_count);
    }
    
    // 策略2: Token估算 - 粗略估算并进一步裁剪
    let mut estimated_tokens = 0;
    let mut keep_index = 0;
    
    for (i, msg) in history.iter().enumerate().rev() {
        let content_len = match &msg.content {
            MessageContent::Text(text) => text.len(),
            MessageContent::ImageUrl(_) => 50,
            MessageContent::Parts(_) => 100,
        };
        estimated_tokens += content_len / 4; // 粗略估算: 4字符≈1token
        
        if estimated_tokens > max_tokens {
            keep_index = i + 1;
            break;
        }
    }
    
    if keep_index > 0 && keep_index < history.len() {
        history.drain(0..keep_index);
    }
}