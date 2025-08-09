# Groq AI Rust SDK

[![Crates.io](https://img.shields.io/crates/v/groqai)](https://crates.io/crates/groqai)
[![Documentation](https://docs.rs/groqai/badge.svg)](https://docs.rs/groqai)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

一个现代化、类型安全的Rust SDK，用于与Groq AI API进行交互。提供简洁的API设计和完整的类型支持。

## ✨ 特性

- 🔒 **类型安全** - 完整的Rust类型系统支持，编译时错误检查
- 🚀 **异步支持** - 基于`tokio`和`reqwest`的高性能异步API
- 📊 **流式传输** - 支持实时流式聊天完成，具有健壮的错误处理
- 🎵 **音频处理** - 音频转录、翻译和语音合成
- 📁 **文件管理** - 完整的文件上传、下载和管理功能
- 🔧 **工具调用** - 支持函数调用和工具使用，提供便捷的助手方法
- 🖼️ **多模态** - 支持文本和图像的多模态输入
- 📦 **批量处理** - 高效的批量任务处理
- 🏗️ **Builder模式** - 清晰的参数设置和配置
- 🛡️ **安全增强** - API密钥验证和环境变量支持
- 📈 **日志概率** - 支持详细的响应概率信息
- 🚨 **错误处理** - 结构化的API错误信息和实用的错误类型

## 📦 安装

```toml
[dependencies]
groqai = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## 🚀 快速开始

### 基本聊天

```rust
use groqai::{GroqClient, ChatMessage, Role, ChatCompletionRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量加载API密钥（推荐）
    let client = GroqClient::from_env()?;
    
    // 或者直接提供API密钥（需要以"gsk_"开头）
    // let client = GroqClient::new("gsk_your-api-key".to_string())?;
    
    let message = ChatMessage::new_text(
        Role::User,
        "你好，请介绍一下你自己".to_string()
    );
    
    let request = ChatCompletionRequest {
        messages: vec![message],
        model: "llama3-8b-8192".to_string(),
        temperature: Some(0.7),
        stream: Some(false),
        ..Default::default()
    };
    
    let response = client.chat_completions(request).await?;
    println!("回复: {}", response.choices[0].message.content);
    Ok(())
}
```

### 流式聊天（增强的错误处理）

```rust
use groqai::{GroqClient, ChatMessage, Role, ChatCompletionRequest};
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GroqClient::from_env()?;
    
    let message = ChatMessage::new_text(
        Role::User,
        "请写一个关于Rust的短诗".to_string()
    );
    
    let request = ChatCompletionRequest {
        messages: vec![message],
        model: "llama3-8b-8192".to_string(),
        stream: Some(true),
        ..Default::default()
    };
    
    let mut stream = client.stream_chat_completions(request).await?;
    
    while let Some(chunk) = stream.try_next().await? {
        if let Some(delta) = &chunk.choices[0].delta {
            print!("{}", delta.content);
        }
    }
    println!();
    Ok(())
}
```

### 工具调用（新增助手方法）

```rust
use groqai::{GroqClient, ChatMessage, Role, Tool, ToolChoice};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GroqClient::from_env()?;
    
    let messages = vec![
        ChatMessage::new_text(Role::User, "北京现在的天气怎么样？".to_string())
    ];
    
    let tools = vec![
        Tool {
            function: Function {
                name: "get_weather".to_string(),
                description: Some("获取指定城市的天气信息".to_string()),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "city": {"type": "string", "description": "城市名称"}
                    },
                    "required": ["city"]
                }),
            },
        }
    ];
    
    // 使用便捷的助手方法
    let response = client.chat_with_tools(
        messages,
        "llama3-8b-8192",
        tools,
        Some(ToolChoice::Auto)
    ).await?;
    
    println!("回复: {}", response.choices[0].message.content);
    Ok(())
}
```

### 多模态消息

```rust
use groqai::{ChatMessage, Role, MessagePart, ImageUrl};

let multimodal_message = ChatMessage::new_multimodal(
    Role::User,
    vec![
        MessagePart::Text {
            text: "请描述这张图片".to_string(),
        },
        MessagePart::ImageUrl {
            image_url: ImageUrl {
                url: "https://example.com/image.jpg".to_string(),
                detail: Some("high".to_string()),
            },
        },
    ]
);
```

### 音频转录（更新：直接文件路径）

```rust
use groqai::{GroqClient, AudioTranscriptionRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GroqClient::from_env()?;
    
    let request = AudioTranscriptionRequest::new(
        "whisper-large-v3".to_string(),
    )
    .language("zh".to_string())
    .prompt("这是一段中文音频".to_string())
    .temperature(0.0);
    
    // 直接传入文件路径，SDK会自动处理文件上传
    let transcription = client.audio_transcription(request, "audio.mp3").await?;
    println!("转录结果: {}", transcription.text);
    Ok(())
}
```

### 文件管理

```rust
use groqai::GroqClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GroqClient::from_env()?;
    
    // 上传文件
    let file = client.upload_file("data.json", "batch").await?;
    println!("文件ID: {}", file.id);
    
    // 列出所有文件
    let files = client.list_files().await?;
    for file in &files.data {
        println!("文件: {} ({} bytes)", file.filename, file.bytes);
    }
    
    // 删除文件
    let delete_result = client.delete_file(&file.id).await?;
    println!("删除成功: {}", delete_result.deleted);
    
    Ok(())
}
```

### 批量处理

```rust
use groqai::GroqClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GroqClient::from_env()?;
    
    // 创建批量任务
    let batch = client.create_batch("file_id", "24h").await?;
    println!("批量任务ID: {}", batch.id);
    
    // 查询批量任务状态
    let batch_status = client.retrieve_batch(&batch.id).await?;
    println!("状态: {}", batch_status.status);
    
    Ok(())
}
```

## 🛠️ 错误处理（增强版）

```rust
use groqai::{GroqClient, GroqError};

match client.chat_completions(request).await {
    Ok(response) => {
        println!("成功: {}", response.choices[0].message.content);
    }
    Err(GroqError::Api(api_error)) => {
        eprintln!("API错误: {} - {}", api_error.status, api_error.message);
        
        // 检查是否为特定类型的错误
        if api_error.is_rate_limit() {
            eprintln!("这是速率限制错误，请稍后重试");
        } else if api_error.is_authentication_error() {
            eprintln!("这是认证错误，请检查API密钥");
        }
        
        // 获取详细的错误信息
        if let Some(details) = api_error.details {
            eprintln!("错误类型: {}", details.error_type);
            eprintln!("错误代码: {}", details.code);
        }
    }
    Err(GroqError::InvalidApiKey(key)) => {
        eprintln!("无效的API密钥格式: {}", key);
    }
    Err(GroqError::StreamParsing(msg)) => {
        eprintln!("流解析错误: {}", msg);
    }
    Err(e) => {
        eprintln!("其他错误: {}", e);
    }
}
```

## 🔐 API密钥管理

### 环境变量（推荐）

```bash
# 设置环境变量
export GROQ_API_KEY="gsk_your-actual-api-key"

# 在代码中使用
let client = GroqClient::from_env()?;
```

### 直接提供

```rust
// 验证格式（必须以"gsk_"开头）
let client = GroqClient::new("gsk_your-api-key".to_string())?;
```

## 📊 日志概率支持

```rust
use groqai::{GroqClient, ChatCompletionRequest};

let request = ChatCompletionRequest {
    messages: vec![message],
    model: "llama3-8b-8192".to_string(),
    logprobs: Some(true),  // 启用日志概率
    top_logprobs: Some(5), // 返回前5个最可能的token
    ..Default::default()
};

let response = client.chat_completions(request).await?;

if let Some(logprobs) = &response.choices[0].logprobs {
    println!("Token概率信息:");
    for (token, prob) in &logprobs.content {
        println!("  {}: {:.4}", token, prob);
    }
}
```

## 📚 API 参考

### 核心类型

- `GroqClient` - 主要的API客户端
- `ChatMessage` - 聊天消息结构
- `Role` - 消息角色枚举
- `MessageContent` - 消息内容类型
- `ToolCall` - 工具调用结构
- `Tool` - 工具定义结构
- `AudioTranscriptionRequest` - 音频转录请求
- `AudioTranslationRequest` - 音频翻译请求
- `LogProbs` - 日志概率信息

### 主要方法

#### 聊天完成
- `chat_completions()` - 聊天完成API
- `stream_chat_completions()` - 流式聊天完成（增强错误处理）

#### 工具调用助手
- `create_tool_call_request()` - 创建工具调用请求
- `chat_with_tools()` - 带工具调用的聊天完成

#### 音频处理
- `audio_transcription()` - 音频转录（直接文件路径）
- `audio_translation()` - 音频翻译（直接文件路径）
- `audio_speech()` - 语音合成

#### 文件管理
- `upload_file()` - 上传文件
- `list_files()` - 列出文件
- `retrieve_file()` - 获取文件信息
- `download_file()` - 下载文件
- `delete_file()` - 删除文件

#### 批量处理
- `create_batch()` - 创建批量任务
- `retrieve_batch()` - 查询批量任务
- `list_batches()` - 列出批量任务
- `cancel_batch()` - 取消批量任务

#### 模型管理
- `get_models()` - 获取可用模型列表

### 构造函数

- `GroqClient::new(api_key)` - 创建客户端（带验证）
- `GroqClient::from_env()` - 从环境变量创建客户端

## 🧪 测试

运行测试套件：

```bash
cargo test
```

测试覆盖：
- 客户端初始化验证
- 基本模型结构创建
- 错误处理逻辑

## 📖 更多示例

查看 `examples/` 目录中的完整示例：

```bash
cargo run --example modern_examples
```

## 📋 版本信息

### v0.1.0 - 初始发布版本
- 🆕 **结构化错误处理** - 完整的API错误信息解析和类型识别
- 🆕 **企业级安全** - API密钥验证和环境变量配置
- 🆕 **健壮流式处理** - 强大的SSE支持和错误恢复
- 🆕 **工具调用集成** - 内置助手方法和简化API
- 🆕 **概率分析** - 完整的token级别概率信息
- 🆕 **专业文档** - 完整的Rustdoc和测试覆盖
- 🎯 **设计理念** - 零配置启动，类型安全，开箱即用

## 🤝 贡献

欢迎贡献！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解贡献指南。

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 📞 支持

- 💬 讨论: [GitHub Discussions](https://github.com/your-username/groqai/discussions)
- 🐛 问题: [GitHub Issues](https://github.com/your-username/groqai/issues)
- 📖 文档: [docs.rs/groqai](https://docs.rs/groqai)
- 🔗 官方API文档: [Groq API Reference](https://console.groq.com/docs/api-reference)

---

**注意**: 这是一个非官方的Groq AI SDK。Groq AI不对此SDK提供官方支持。
