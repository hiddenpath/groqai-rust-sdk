# Groq AI Rust SDK v0.1.0 发布说明

## 🎉 欢迎使用 Groq AI Rust SDK！

这是我们的第一个正式版本，专为生产环境设计，提供了完整的Groq AI API集成功能。

## ✨ 核心特性

### 🔍 智能错误处理
- **结构化错误信息** - 不再只是状态码，而是详细的错误上下文
- **错误类型识别** - 自动识别速率限制、认证错误等常见问题
- **友好错误提示** - 提供具体的解决建议

### 🔐 企业级安全
- **API密钥验证** - 严格的格式检查（必须以 `gsk_` 开头）
- **环境变量支持** - 安全的配置管理，避免硬编码密钥
- **类型安全** - 完整的Rust类型系统保护

### 🚀 流式响应处理
- **健壮SSE支持** - 处理各种格式的Server-Sent Events
- **自动错误恢复** - 智能处理流式数据中的格式问题
- **UTF-8验证** - 确保数据完整性

### 🔧 工具调用集成
- **内置助手方法** - `create_tool_call_request()` 和 `chat_with_tools()`
- **简化API** - 减少样板代码，提高开发效率
- **完整支持** - 支持所有Groq工具调用功能

### 📊 概率分析
- **Token级别概率** - 详细的响应内容概率信息
- **Top-K分析** - 支持多种概率分析模式
- **结构化数据** - 易于处理和分析的概率信息

## 🚀 快速开始

### 环境变量方式（推荐）
```bash
export GROQ_API_KEY="gsk_your_api_key_here"
```

```rust
use groqai::GroqClient;

let client = GroqClient::from_env()?;
```

### 直接初始化
```rust
use groqai::GroqClient;

let client = GroqClient::new("gsk_your_api_key_here".to_string())?;
```

## 📚 完整文档

- **API文档**: 运行 `cargo doc --open` 查看完整Rustdoc
- **使用示例**: 查看 `examples/` 目录
- **测试覆盖**: 运行 `cargo test` 验证功能

## 🎯 设计理念

这个版本遵循以下设计原则：

1. **零配置启动** - 环境变量自动配置，开箱即用
2. **类型安全** - 完整的Rust类型系统，编译时错误检查
3. **错误友好** - 详细的错误信息和处理建议
4. **性能优先** - 高效的异步处理和内存管理
5. **开发者体验** - 直观的API设计和丰富的文档

## 🔮 未来规划

v0.2.0 将包含：
- 集成测试套件
- 性能基准测试
- 更多使用示例
- 自动重试机制
- 请求监控和日志

## 🤝 反馈和支持

- **GitHub Issues**: 报告问题或建议功能
- **GitHub Discussions**: 讨论使用方法和最佳实践
- **贡献指南**: 欢迎提交Pull Request

---

**注意**: 这是一个非官方的Groq AI SDK。Groq AI不对此SDK提供官方支持。

**许可证**: MIT License - 查看 [LICENSE](LICENSE) 文件了解详情。
