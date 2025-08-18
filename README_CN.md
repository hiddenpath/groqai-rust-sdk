# GroqAI - Rust 客户端 SDK 库

[![Crates.io](https://img.shields.io/crates/v/groqai.svg)](https://crates.io/crates/groqai)
[![Documentation](https://docs.rs/groqai/badge.svg)](https://docs.rs/groqai)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)

一个现代化、类型安全的 [Groq API](https://groq.com/) Rust SDK，具有企业级特性，提供闪电般快速的 AI 推理能力，配备全面的错误处理和内置弹性机制。

## 特性

- 🚀 **高性能** - 基于异步构建，支持 async/await 和高效的 HTTP 传输
- 💬 **聊天完成** - 支持流式和非流式对话，具有高级消息类型
- 🎵 **音频处理** - 使用 Whisper 模型进行转录和翻译，支持文件和 URL
- 📁 **文件管理** - 完整的文件生命周期管理（上传、列表、检索、删除）
- 🔄 **批处理** - 大规模任务的高效批量操作，支持状态监控
- 🤖 **模型信息** - 获取可用模型及其详细功能
- 🎯 **微调** - 支持监督学习的自定义模型训练
- 🛡️ **企业级错误处理** - 全面的错误类型、自动重试和优雅降级
- 📊 **智能速率限制** - 内置速率限制，支持指数退避和 retry-after 头
- 🔧 **灵活配置** - 可自定义超时、代理、基础 URL 和传输设置
- 🔒 **类型安全** - 强类型 API，提供编译时保证
- 🌐 **代理支持** - 完整的 HTTP/HTTPS 代理支持，适用于企业环境
- 📝 **丰富消息类型** - 支持文本、图像和多部分消息
- 🔄 **对话管理** - 内置对话历史管理，支持令牌优化

## 快速开始

### 安装

在您的 `Cargo.toml` 中添加：

```toml
[dependencies]
groqai = "0.1.10"
tokio = { version = "1.47", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

或通过 cargo 安装：

```bash
cargo add groqai
cargo add tokio --features full
```

### 基本用法

#### 使用环境变量（推荐）

**⚠️ 前置条件：先设置环境变量！**

```bash
# 必需
export GROQ_API_KEY="gsk_your_api_key_here"

# 可选
export GROQ_PROXY_URL="http://proxy.example.com:8080"
export GROQ_TIMEOUT_SECS="60"
```

```rust
// 选项1：导入特定类型（推荐用于应用程序）
use groqai::{GroqClient, ChatMessage, Role};

// 选项2：使用 prelude 便捷导入（推荐用于学习）
// use groqai::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量创建客户端（需要 GROQ_API_KEY）
    let client = GroqClient::new()?;
    
    // 创建聊天完成
    let messages = vec![
        ChatMessage::new_text(Role::User, "用简单的术语解释量子计算")
    ];
    
    let response = client
        .chat("llama-3.1-70b-versatile")
        .messages(messages)
        .temperature(0.7)
        .max_completion_tokens(500)
        .send()
        .await?;
    
    println!("回复: {}", response.choices[0].message.content);
    Ok(())
}
```

#### 直接使用 API 密钥

```rust
use groqai::prelude::*;  // 便捷导入常用类型

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 使用 API 密钥创建客户端
    let client = GroqClient::with_api_key("gsk_your_api_key_here")?;
    
    let messages = vec![
        ChatMessage::new_text(Role::User, "你好！")
    ];
    
    let response = client
        .chat("llama-3.1-70b-versatile")
        .messages(messages)
        .send()
        .await?;
    
    println!("回复: {}", response.choices[0].message.content);
    Ok(())
}
```

## 客户端创建方法

GroqAI 提供四种灵活的客户端创建方式：

### 1. 环境变量（推荐）
```bash
# 首先设置必需的环境变量
export GROQ_API_KEY="gsk_your_api_key_here"
```
```rust
// 最简单的方式，适用于生产环境
let client = GroqClient::new()?;  // 从 GROQ_API_KEY 读取
```

### 2. 直接 API 密钥
```rust
// 快速设置，适用于开发测试
let client = GroqClient::with_api_key("gsk_your_key")?;
```

### 3. 显式环境变量
```rust
// 显式使用环境变量
let client = GroqClient::from_env()?;
```

### 4. 构建器模式（高级）
```rust
// 完全控制所有设置
let client = GroqClientBuilder::new("gsk_your_key".to_string())?
    .timeout(Duration::from_secs(60))
    .proxy(proxy)
    .build()?;
```

### 环境变量说明

```bash
# 必需
export GROQ_API_KEY="gsk_your_api_key_here"

# 可选
export GROQ_PROXY_URL="http://proxy.example.com:8080"
export GROQ_TIMEOUT_SECS="60"  # 默认 30 秒
```

## 导入模式

GroqAI 提供灵活的导入选项以适应不同用例：

### 1. Prelude 导入（推荐用于学习）
```rust
use groqai::prelude::*;
// 导入: GroqClient, GroqError, ChatMessage, Role, MessageContent, KnownModel, ChatCompletionResponse
```

### 2. 特定导入（推荐用于应用程序）
```rust
use groqai::{GroqClient, ChatMessage, Role, GroqError};
```

### 3. 粒度导入（用于库开发）
```rust
use groqai::{
    GroqClient, GroqClientBuilder,
    ChatMessage, Role, MessageContent,
    ChatCompletionResponse, GroqError,
};
```

**根据需求选择：**
- 📚 **学习/原型开发**: 使用 `prelude::*` 获得便利
- 🏢 **应用程序**: 使用特定导入获得清晰性
- 📦 **库开发**: 使用粒度导入避免冲突

## API 参考

### 聊天完成

#### 非流式聊天

```rust
use groqai::{GroqClientBuilder, ChatMessage, Role};

let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?.build()?;

let messages = vec![
    ChatMessage::new_text(Role::System, "你是一个有用的助手。"),
    ChatMessage::new_text(Role::User, "法国的首都是什么？"),
];

let response = client
    .chat("llama-3.1-70b-versatile")
    .messages(messages)
    .temperature(0.7)
    .send()
    .await?;

println!("{}", response.choices[0].message.content);
```

#### 流式聊天

```rust
use futures::StreamExt;

let mut stream = client
    .chat("llama-3.1-70b-versatile")
    .messages(messages)
    .stream(true)
    .send_stream()
    .await?;

while let Some(chunk) = stream.next().await {
    match chunk {
        Ok(chunk) => {
            if let Some(content) = &chunk.choices[0].delta.content {
                print!("{}", content);
            }
        }
        Err(e) => eprintln!("错误: {}", e),
    }
}
```

### 音频处理

#### 转录

```rust
use groqai::AudioTranscriptionRequest;
use std::path::PathBuf;

let request = AudioTranscriptionRequest {
    file: Some(PathBuf::from("audio.mp3")),
    url: None,
    model: "whisper-large-v3".to_string(),
    language: Some("zh".to_string()),
    prompt: None,
    response_format: Some("json".to_string()),
    temperature: Some(0.0),
    timestamp_granularities: None,
};

let transcription = client.audio().transcribe(request).await?;
println!("转录结果: {}", transcription.text);
```

#### 翻译

```rust
use groqai::AudioTranslationRequest;

let request = AudioTranslationRequest {
    file: Some(PathBuf::from("chinese_audio.mp3")),
    url: None,
    model: "whisper-large-v3".to_string(),
    prompt: None,
    response_format: Some("json".to_string()),
    temperature: Some(0.0),
};

let translation = client.audio().translate(request).await?;
println!("翻译结果: {}", translation.text);
```

### 文件管理

```rust
use groqai::FileCreateRequest;
use std::path::PathBuf;

// 上传文件
let request = FileCreateRequest::new(
    PathBuf::from("training_data.jsonl"),
    "batch".to_string()
)?;
let file = client.files().create(request).await?;

// 列出文件
let files = client.files().list().await?;
for file in files.data {
    println!("文件: {} ({})", file.filename, file.purpose);
}

// 检索文件
let file = client.files().retrieve("file_id".to_string()).await?;

// 删除文件
let deletion = client.files().delete("file_id".to_string()).await?;
```

### 批处理

```rust
use groqai::BatchCreateRequest;

// 创建批处理任务
let request = BatchCreateRequest {
    input_file_id: "file_abc123".to_string(),
    endpoint: "/chat/completions".to_string(),
    completion_window: "24h".to_string(),
    metadata: None,
};

let batch = client.batches().create(request).await?;
println!("批处理已创建: {}", batch.id);

// 检查批处理状态
let batch = client.batches().retrieve(batch.id).await?;
println!("状态: {}", batch.status);

// 列出批处理
let batches = client.batches().list(None, Some(10)).await?;
```

### 模型信息

```rust
// 列出可用模型
let models = client.models().list().await?;
for model in models.data {
    println!("模型: {} - {}", model.id, model.owned_by);
}

// 获取模型详情
let model = client.models().retrieve("llama-3.1-70b-versatile".to_string()).await?;
println!("上下文窗口: {} 个令牌", model.context_window);
```

## 配置

### 环境变量

客户端可以通过环境变量进行配置：

```bash
# 必需
export GROQ_API_KEY="gsk_your_api_key_here"

# 可选
export GROQ_PROXY_URL="http://proxy.example.com:8080"  # 或 HTTPS_PROXY/HTTP_PROXY
export GROQ_TIMEOUT_SECS="60"  # 默认: 30
```

```rust
use groqai::GroqClient;

// 自动使用环境变量
let client = GroqClient::new()?;
```

### 使用构建器进行高级配置

```rust
use groqai::GroqClientBuilder;
use std::time::Duration;
use url::Url;

let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?
    .base_url(Url::parse("https://api.groq.com/openai/v1/")?)
    .timeout(Duration::from_secs(60))
    .build()?;
```

### 使用代理

```rust
use groqai::GroqClientBuilder;

let proxy = reqwest::Proxy::http("http://proxy.example.com:8080")?;
let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?
    .proxy(proxy)
    .build()?;
```

## 错误处理

库通过 `GroqError` 枚举提供全面的错误处理：

```rust
use groqai::GroqError;

match client.chat("model").messages(messages).send().await {
    Ok(response) => println!("成功: {}", response.choices[0].message.content),
    Err(GroqError::RateLimited) => println!("速率受限，请稍后重试"),
    Err(GroqError::InvalidApiKey(_)) => println!("无效的 API 密钥"),
    Err(GroqError::Api(api_error)) => println!("API 错误: {}", api_error.error.message),
    Err(e) => println!("其他错误: {}", e),
}
```

## 支持的模型

SDK 支持所有当前的 Groq 模型，具有内置类型安全：

### 聊天模型
- **Llama 3.1 系列**: 
  - `llama-3.1-8b-instant` - 简单任务的快速响应
  - `llama-3.1-70b-versatile` - 平衡的性能和能力
  - `llama-3.1-405b-reasoning` - 高级推理和复杂任务
  - `llama-3.3-70b-versatile` - 具有增强功能的最新模型
- **Mixtral**: `mixtral-8x7b-32768` - 大上下文窗口，适用于复杂对话
- **Gemma**: `gemma2-9b-it` - 高效的指令调优模型
- **Qwen**: `qwen2.5-72b-instruct` - 多语言能力

### 音频模型
- **Whisper**: `whisper-large-v3` - 最先进的语音识别和翻译

### 模型选择助手
```rust
use groqai::KnownModel;

// 类型安全的模型选择
let model = KnownModel::Llama3_1_70bVersatile;
let response = client.chat(&model.to_string()).send().await?;
```

## 速率限制

客户端包含内置的速率限制和指数退避：

```rust
// 速率限制会自动处理
let response = client.chat("model").messages(messages).send().await?;
```

## 高级功能

### 多模态消息
```rust
use groqai::{ChatMessage, Role, MessageContent, ImageUrl};

let messages = vec![
    ChatMessage {
        role: Role::User,
        content: MessageContent::Parts(vec![
            MessagePart::Text { text: "这张图片里有什么？".to_string() },
            MessagePart::ImageUrl { 
                image_url: ImageUrl::new("https://example.com/image.jpg") 
            },
        ]),
        name: None,
        tool_calls: None,
        tool_call_id: None,
    }
];
```

### 对话历史管理
```rust
// 内置对话管理，支持令牌优化
let mut conversation = Vec::new();
conversation.push(ChatMessage::new_text(Role::User, "你好"));

// 自动历史修剪以保持在令牌限制内
trim_conversation_history(&mut conversation, 15, 18000);
```

### 企业代理配置

#### 使用环境变量
```bash
export GROQ_API_KEY="gsk_your_api_key"
export GROQ_PROXY_URL="http://username:password@corporate-proxy:8080"
export GROQ_TIMEOUT_SECS="120"
```

```rust
use groqai::GroqClient;

// 自动从环境变量使用代理
let client = GroqClient::new()?;
```

#### 使用构建器模式
```rust
use groqai::GroqClientBuilder;
use reqwest::Proxy;
use std::time::Duration;

let proxy = Proxy::all("http://corporate-proxy:8080")?
    .basic_auth("username", "password");

let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?
    .proxy(proxy)
    .timeout(Duration::from_secs(120))
    .build()?;
```

## 示例

查看 `examples/` 目录获取综合示例：

- `cli_chat.rs` - 支持流式的交互式 CLI 聊天应用
- `chat_completion.rs` - 基本聊天完成
- `streaming_chat.rs` - 流式响应
- `audio_transcription.rs` - 音频处理
- `batch_processing.rs` - 批处理操作
- `file_management.rs` - 文件操作
- `model_info.rs` - 模型信息和功能
- `client_convenience.rs` - 便捷方法演示
- `client_creation_methods.rs` - 客户端创建方法完整指南
- `import_patterns.rs` - 不同导入模式和最佳实践
- `environment_setup.rs` - 环境变量设置和故障排除

## 要求

- Rust 1.70 或更高版本
- 有效的 Groq API 密钥（在 [console.groq.com](https://console.groq.com/) 获取）

## 项目状态

此 SDK 正在积极维护且已可用于生产环境。当前版本：**0.1.10**

### 路线图

- ✅ 聊天完成（流式和非流式）
- ✅ 音频转录和翻译
- ✅ 文件管理
- ✅ 批处理
- ✅ 模型信息
- ✅ 微调支持
- ✅ 企业功能（代理、速率限制）
- 🔄 函数调用（开发中）
- 📋 视觉 API 增强
- 📋 高级流式功能

## 贡献

欢迎贡献！请随时提交 Pull Request。对于重大更改，请先开启 issue 讨论您想要更改的内容。

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 致谢

- [Groq](https://groq.com/) 提供闪电般快速的 AI 推理 API
- Rust 社区提供优秀的异步和 HTTP 库
- 帮助改进此 SDK 的贡献者和用户

## 架构

SDK 采用模块化架构构建：

- **传输层** (`transport.rs`) - 具有重试逻辑和速率限制的 HTTP 客户端
- **API 模块** (`api/`) - 每个 Groq 服务的端点特定实现
- **类型系统** (`types.rs`) - 强类型的请求/响应结构
- **错误处理** (`error.rs`) - 带上下文的全面错误类型
- **速率限制** (`rate_limit.rs`) - 具有指数退避的智能速率限制
- **客户端构建器** (`client.rs`) - 灵活的客户端配置

## 性能考虑

- **异步/等待**: 基于 Tokio 构建，提供高性能异步操作
- **连接池**: 重用 HTTP 连接以获得更好的性能
- **流式处理**: 为实时应用提供高效流式处理
- **内存管理**: 针对低内存占用进行优化
- **速率限制**: 通过智能退避防止 API 配额耗尽

## 安全特性

- **API 密钥验证**: 在构建时验证 API 密钥格式
- **仅 HTTPS**: 所有通信都使用 TLS 加密
- **代理支持**: 完全支持企业代理环境
- **错误清理**: 敏感数据不会记录在错误消息中

## 测试

SDK 包含全面的测试：

```bash
# 运行所有测试
cargo test

# 运行特定测试模块
cargo test tests::chat
cargo test tests::audio
cargo test tests::files
```

## 支持

- 📖 [文档](https://docs.rs/groqai)
- 🐛 [问题跟踪](https://github.com/hiddenpath/groqai-rust-sdk/issues)
- 💬 [讨论](https://github.com/hiddenpath/groqai-rust-sdk/discussions)
- 📧 [作者](mailto:alex.wang@msn.com)

---

**注意**: 这是一个非官方的客户端 SDK。如需官方支持，请参考 [Groq 文档](https://console.groq.com/docs)。