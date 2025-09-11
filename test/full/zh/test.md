<a id="readme-top"></a>

<!-- [![Contributors][contributors-shield]][contributors-url] -->
[![分叉][forks-shield]][forks-url]
[![收藏][stars-shield]][stars-url]
[![议题][issues-shield]][issues-url]
[![MIT 许可证][license-shield]][license-url]
<!-- [![LinkedIn][linkedin-shield]][linkedin-url] -->


<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/lfnovo/open-notebook">
    <img src="docs/assets/hero.svg" alt="Logo">
  </a>

  <h3 align="center">Open Notebook</h3>

  <p align="center">
    一款开源、注重隐私的 Google Notebook LM 替代品！
    <br /><strong>加入我们的 <a href="https://discord.gg/37XJPXfz2w">Discord 服务器 </a> 获取帮助、分享工作流创意和提出功能建议！</strong>
    <br />
    <a href="https://www.open-notebook.ai"><strong>访问我们的网站 »</strong></a>
    <br />
    <br />
    <a href="docs/getting-started/index.md">📚 开始使用</a>
    ·
    <a href="docs/user-guide/index.md">📖 用户指南</a>
    ·
    <a href="docs/features/index.md">✨ 功能特性</a>
    ·
    <a href="docs/deployment/index.md">🚀 部署</a>
  </p>
</div>

## 📢 Open Notebook 正处于非常活跃的开发阶段

> Open Notebook 正处于活跃的开发阶段！我们进展迅速，每周都会进行改进。在这个激动人心的阶段，您的反馈对我来说无比宝贵，它激励我不断完善和构建这款出色的工具。如果您觉得这个项目有用，请随时为它点亮星标，并毫不犹豫地提出任何问题或建议。我非常期待看到您将如何使用它，以及您会为这个项目带来什么样的想法！让我们一起创造卓越！ 🚀

## 关于项目

![新建笔记本](docs/assets/asset_list.png)

一款开源、注重隐私的 Google Notebook LM 替代品。既然我们能够掌控自己的研究工作流，为何还要将更多数据交给 Google？

在一个由人工智能主导的世界里，拥有思考🧠和获取新知识💡的能力，不应是少数人的特权，也不应受限于单一的供应商。

**Open Notebook 赋予您以下能力:**
- 🔒 **掌控您的数据** - 确保您的研究私密且安全
- 🤖 **选择您的 AI 模型** - 支持超过 16 家供应商，包括 OpenAI、Anthropic、Ollama、LM Studio 等
- 📚 **组织多模态内容** - 支持 PDF、视频、音频、网页等多种格式
- 🎙️ **生成专业级播客** - 先进的多人对话播客生成功能
- 🔍 **智能搜索** - 对您的所有内容进行全文和向量搜索
- 💬 **结合上下文对话** - 由您的研究资料驱动的 AI 对话

访问 [https: //www.open-notebook.ai](https://www.open-notebook.ai) 了解更多关于我们项目的信息

## 🆚 Open Notebook vs. Google Notebook LM

| 功能 | Open Notebook | Google Notebook LM | 优势 |
|---------|---------------|--------------------|-----------|
| **隐私与控制** | 自主托管，数据归您所有 | 仅限 Google Cloud | 完全的数据主权 |
| **AI 供应商选择** | 超过 16 家供应商 (OpenAI, Anthropic, Ollama, LM Studio 等) | 仅限 Google 模型 | 灵活性与成本优化 |
| **播客发言人** | 1-4 位发言人，可自定义配置 | 仅限 2 位发言人 | 极高的灵活性 |
| **上下文控制** | 3 个精细层级 | 全有或全无 | 隐私与性能调优 |
| **内容转换** | 自定义与内置转换 | 选项有限 | 无限的处理能力 |
| **API 访问** | 完整的 REST API | 无 API | 完全自动化 |
| **部署** | Docker、云端或本地 | 仅限 Google 托管 | 随处部署 |
| **引文** | 全面的来源引用 | 基础的参考文献 | 保障研究的完整性 |
| **定制化** | 开源，完全可定制 | 封闭系统 | 无限的可扩展性 |
| **成本** | 仅为 AI 使用付费 | 月度订阅 + 使用费 | 透明且可控 |

**为何选择 Open Notebook？**
- 🔒 **隐私优先**: 您的敏感研究将保持完全私密
- 💰 **成本控制**: 选择更经济的 AI 供应商，或通过 Ollama 在本地运行
- 🎙️ **更出色的播客**: 完全的脚本控制和多发言人灵活性，优于仅限 2 人的深度探讨格式
- 🔧 **无限定制**: 根据需要修改、扩展和集成
- 🌐 **无供应商锁定**: 随时切换供应商、随处部署、拥有您的数据

### 构建技术

[![Python][Python]][Python-url] [![SurrealDB][SurrealDB]][SurrealDB-url] [![LangChain][LangChain]][LangChain-url] [![Streamlit][Streamlit]][Streamlit-url]

## 🚀 快速入门

准备好试用 Open Notebook 了吗？请选择您偏好的方式:

### ⚡ 即时设置 (推荐)
```bash
# 为您的 Open Notebook 安装创建一个新目录
mkdir open-notebook
cd open-notebook

# 使用 Docker - 2 分钟内即可开始
docker run -d \
  --name open-notebook \
  -p 8502:8502 -p 5055:5055 \
  -v ./notebook_data:/app/data \
  -v ./surreal_data:/mydata \
  -e OPENAI_API_KEY=your_key \
  lfnovo/open_notebook:latest-single
```

**将会创建什么:**
```
open-notebook/
├── notebook_data/     # Your notebooks and research content
└── surreal_data/      # Database files
```

**访问您的安装实例:**
- **🖥️ 主界面**: http: //localhost: 8502 (Streamlit UI)
- **🔧 API 访问**: http: //localhost: 5055 (REST API)
- **📚 API 文档**: http: //localhost: 5055/docs (交互式 Swagger UI)

> **⚠️ 重要提示**:
> 1. **从专用文件夹运行**: 请在一个新的 `open-notebook` 文件夹内创建并运行，以确保您的数据卷被妥善组织
> 2. **数据卷持久化**: 数据卷 (`-v ./notebook_data:/app/data` 和 `-v ./surreal_data:/mydata`) 对于在容器重启后保留数据至关重要。没有它们，当容器停止时，您将丢失所有的笔记本和研究资料。

### 🛠️ 完整安装
适用于开发或定制:
```bash
git clone https://github.com/lfnovo/open-notebook
cd open-notebook
make start-all
```

### 📖 需要帮助？
- **🤖 AI 安装助手**: 我们构建了一个 [ 定制版 GPT 来帮助您安装 Open Notebook](https://chatgpt.com/g/g-68776e2765b48191bd1bae3f30212631-open-notebook-installation-assistant) - 它将引导您完成每一步！
- ** 初次使用 Open Notebook？** 请从我们的 [ 入门指南 ](docs/getting-started/index.md) 开始
- ** 需要安装帮助？** 请查阅我们的 [ 安装指南 ](docs/getting-started/installation.md)
- ** 想看实际操作？** 请尝试我们的 [ 快速入门教程 ](docs/getting-started/quick-start.md)

## 供应商支持矩阵

得益于 [Esperanto](https://github.com/lfnovo/esperanto) 库，我们原生支持以下供应商！

| 供应商     | LLM 支持 | Embedding 支持 | 语音转文本 | 文本转语音 |
|--------------|-------------|------------------|----------------|----------------|
| OpenAI       | ✅          | ✅               | ✅             | ✅             |
| Anthropic    | ✅          | ❌               | ❌             | ❌             |
| Groq         | ✅          | ❌               | ✅             | ❌             |
| Google (GenAI) | ✅          | ✅               | ❌             | ✅             |
| Vertex AI    | ✅          | ✅               | ❌             | ✅             |
| Ollama       | ✅          | ✅               | ❌             | ❌             |
| Perplexity   | ✅          | ❌               | ❌             | ❌             |
| ElevenLabs   | ❌          | ❌               | ✅             | ✅             |
| Azure OpenAI | ✅          | ✅               | ❌             | ❌             |
| Mistral      | ✅          | ✅               | ❌             | ❌             |
| DeepSeek     | ✅          | ❌               | ❌             | ❌             |
| Voyage       | ❌          | ✅               | ❌             | ❌             |
| xAI          | ✅          | ❌               | ❌             | ❌             |
| OpenRouter   | ✅          | ❌               | ❌             | ❌             |
| OpenAI 兼容* | ✅          | ❌               | ❌             | ❌             |

*支持 LM Studio 及任何与 OpenAI 兼容的端点

## ✨ 主要特性

### 核心能力
- **🔒 隐私优先**: 数据尽在掌握，无需依赖云端
- **🎯 多笔记本管理**: 无缝管理多个研究项目
- **📚 通用内容支持**: 支持 PDF、视频、音频、网页、Office 文档等多种格式
- **🤖 多模型 AI 支持**: 支持超过 16 家提供商，包括 OpenAI、Anthropic、Ollama、Google、LM Studio 等
- **🎙️ 专业播客生成**: 通过“剧集配置”创建高级多主播播客
- **🔍 智能搜索**: 对您的所有内容进行全文和向量搜索
- **💬 上下文感知聊天**: 基于您的研究资料进行 AI 对话
- **📝 AI 辅助笔记**: 生成洞见或手动撰写笔记

### 高级功能
- **⚡ 推理模型支持**: 全面支持 DeepSeek-R1、Qwen3 等思维模型
- **🔧 内容转换**: 强大的可自定义操作，用于总结和提取洞见
- **🌐 全面的 REST API**: 通过完整的编程访问实现自定义集成 <a href="http://localhost:5055/docs">![API 文档 ](https://img.shields.io/badge/API-Documentation-blue?style=flat-square)</a>
- **🔐 可选密码保护**: 通过身份验证保护公开部署的安全
- **📊 细粒度上下文控制**: 精准选择与 AI 模型共享的内容
- **📎 引用标注**: 获取答案时附带准确的来源引用

### 三栏式界面
1. **源文件**: 管理您的所有研究资料
2. **笔记**: 创建手动或 AI 生成的笔记
3. **聊天**: 以您的内容为上下文与 AI 进行对话

[![查看我们的播客示例](https://img.youtube.com/vi/D-760MlGwaI/0.jpg)](https://www.youtube.com/watch?v=D-760MlGwaI)

## 📚 文档

### 入门指南
- **[📖 简介](docs/getting-started/introduction.md)** - 了解 Open Notebook 的功能
- **[⚡ 快速入门](docs/getting-started/quick-start.md)** - 5 分钟内完成设置并开始运行
- **[🔧 安装](docs/getting-started/installation.md)** - 全面的安装指南
- **[🎯 您的第一个笔记本](docs/getting-started/first-notebook.md)** - 分步教程

### 用户指南
- **[📱 界面概览](docs/user-guide/interface-overview.md)** - 了解界面布局
- **[📚 笔记本](docs/user-guide/notebooks.md)** - 组织您的研究
- **[📄 源文件](docs/user-guide/sources.md)** - 管理内容类型
- **[📝 笔记](docs/user-guide/notes.md)** - 创建和管理笔记
- **[💬 聊天](docs/user-guide/chat.md)** - AI 对话
- **[🔍 搜索](docs/user-guide/search.md)** - 查找信息

### 高级主题
- **[🎙️ 播客生成](docs/features/podcasts.md)** - 创建专业播客
- **[🔧 内容转换](docs/features/transformations.md)** - 自定义内容处理
- **[🤖 AI 模型](docs/features/ai-models.md)** - AI 模型配置
- **[🔧 REST API 参考](docs/development/api-reference.md)** - 完整的 API 文档
- **[🔐 安全](docs/deployment/security.md)** - 密码保护与隐私
- **[🚀 部署](docs/deployment/index.md)** - 适用于所有场景的完整部署指南

<p align="right">(<a href="#readme-top">返回顶部</a>)</p>

## 🗺️ 路线图

### 即将推出的功能
- **React 前端**: 基于 React 的现代化前端，将取代 Streamlit
- **前端实时更新**: 实时 UI 更新，带来更流畅的体验
- **异步处理**: 通过异步内容处理提升 UI 响应速度
- **跨笔记本源文件**: 在不同项目间复用研究资料
- **书签集成**: 连接您喜爱的书签应用

### 近期完成 ✅
- **全面的 REST API**: 对所有功能提供完整的编程访问
- **多模型支持**: 支持超过 16 家 AI 提供商，包括 OpenAI、Anthropic、Ollama、LM Studio
- **高级播客生成器**: 通过“剧集配置”创建专业的多主播播客
- **内容转换**: 强大的可自定义操作，用于内容处理
- **增强的引用标注**: 改进布局，更精细地控制来源引用
- **多聊天会话**: 在笔记本内管理不同的对话

查看 [ 公开的 issue](https://github.com/lfnovo/open-notebook/issues)，获取拟议功能和已知问题的完整列表。

<p align="right">(<a href="#readme-top">返回顶部</a>)</p>


## 🤝 社区与贡献

### 加入社区
- 💬 **[Discord 服务器](https://discord.gg/37XJPXfz2w)** - 获取帮助、分享想法、并与其他用户交流
- 🐛 **[GitHub Issues](https://github.com/lfnovo/open-notebook/issues)** - 报告错误和请求功能
- ⭐ **为本仓库点亮 Star** - 表达您的支持，并帮助他人发现 Open Notebook

### 参与贡献
我们欢迎各种贡献！我们尤其需要以下方面的帮助:
- **前端开发**: 帮助构建基于 React 的现代化 UI（ 计划取代当前的 Streamlit 界面 ）
- **测试与错误修复**: 让 Open Notebook 更加稳健
- **功能开发**: 共同打造最酷的研究工具
- **文档撰写**: 改进指南和教程

**当前技术栈**: Python、FastAPI、SurrealDB、Streamlit
**未来路线图**: React 前端、增强的实时更新

请参阅我们的 [ 贡献指南 ](CONTRIBUTING.md)，了解如何开始的详细信息。

<p align="right">(<a href="#readme-top">返回顶部</a>)</p>


## 📄 许可证

Open Notebook 采用 MIT 许可证。详情请参阅 [LICENSE](LICENSE) 文件。

## 📞 联系方式

**Luis Novo** - [@lfnovo](https://twitter.com/lfnovo)

**社区支持**:
- 💬 [Discord 服务器 ](https://discord.gg/37XJPXfz2w) - 获取帮助、分享想法、并与用户交流
- 🐛 [GitHub Issues](https://github.com/lfnovo/open-notebook/issues) - 报告错误和请求功能
- 🌐 [ 网站 ](https://www.open-notebook.ai) - 了解关于本项目的更多信息

## 🙏 致谢

Open Notebook 的构建离不开众多优秀的开源项目:

* **[Podcast Creator](https://github.com/lfnovo/podcast-creator)** - 高级播客生成功能
* **[Surreal Commands](https://github.com/lfnovo/surreal-commands)** - 后台任务处理
* **[Content Core](https://github.com/lfnovo/content-core)** - 内容处理与管理
* **[Esperanto](https://github.com/lfnovo/esperanto)** - 多提供商 AI 模型抽象层
* **[Docling](https://github.com/docling-project/docling)** - 文档处理与解析

<p align="right">(<a href="#readme-top">返回顶部</a>)</p>


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/lfnovo/open-notebook.svg?style=for-the-badge
[contributors-url]: https://github.com/lfnovo/open-notebook/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/lfnovo/open-notebook.svg?style=for-the-badge
[forks-url]: https://github.com/lfnovo/open-notebook/network/members
[stars-shield]: https://img.shields.io/github/stars/lfnovo/open-notebook.svg?style=for-the-badge
[stars-url]: https://github.com/lfnovo/open-notebook/stargazers
[issues-shield]: https://img.shields.io/github/issues/lfnovo/open-notebook.svg?style=for-the-badge
[issues-url]: https://github.com/lfnovo/open-notebook/issues
[license-shield]: https://img.shields.io/github/license/lfnovo/open-notebook.svg?style=for-the-badge
[license-url]: https://github.com/lfnovo/open-notebook/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/lfnovo
[product-screenshot]: images/screenshot.png
[Streamlit]: https://img.shields.io/badge/Streamlit-FF4B4B?style=for-the-badge&logo=streamlit&logoColor=white
[Streamlit-url]: https://streamlit.io/
[Python]: https://img.shields.io/badge/Python-3776AB?style=for-the-badge&logo=python&logoColor=white
[Python-url]: https://www.python.org/
[LangChain]: https://img.shields.io/badge/LangChain-3A3A3A?style=for-the-badge&logo=chainlink&logoColor=white
[LangChain-url]: https://www.langchain.com/
[SurrealDB]: https://img.shields.io/badge/SurrealDB-FF5E00?style=for-the-badge&logo=databricks&logoColor=white
[SurrealDB-url]: https://surrealdb.com/


<pre>
need tran
  </pre>