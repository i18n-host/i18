<a id="readme-top"></a>

<!-- [![Contributors][contributors-shield]][contributors-url] -->
[![分叉][forks-shield]][forks-url]
[![收藏][stars-shield]][stars-url]
[![議題][issues-shield]][issues-url]
[![MIT 許可證][license-shield]][license-url]
<!-- [![LinkedIn][linkedin-shield]][linkedin-url] -->


<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/lfnovo/open-notebook">
    <img src="docs/assets/hero.svg" alt="Logo">
  </a>

  <h3 align="center">Open Notebook</h3>

  <p align="center">
    一款開源、注重隱私的 Google Notebook LM 替代品！
    <br /><strong>加入我們的 <a href="https://discord.gg/37XJPXfz2w">Discord 服務器 </a> 獲取幫助、分享工作流創意和提出功能建議！</strong>
    <br />
    <a href="https://www.open-notebook.ai"><strong>訪問我們的網站 »</strong></a>
    <br />
    <br />
    <a href="docs/getting-started/index.md">📚 開始使用</a>
    ·
    <a href="docs/user-guide/index.md">📖 用戶指南</a>
    ·
    <a href="docs/features/index.md">✨ 功能特性</a>
    ·
    <a href="docs/deployment/index.md">🚀 部署</a>
  </p>
</div>

## 📢 Open Notebook 正處於非常活躍的開發階段

> Open Notebook 正處於活躍的開發階段！我們進展迅速，每周都會進行改進。在這個激動人心的階段，您的反饋對我來說無比寶貴，它激勵我不斷完善和構建這款出色的工具。如果您覺得這個項目有用，請隨時為它點亮星標，并毫不猶豫地提出任何問題或建議。我非常期待看到您將如何使用它，以及您會為這個項目帶來什麼樣的想法！讓我們一起創造卓越！ 🚀

## 關於項目

![新建筆記本](docs/assets/asset_list.png)

一款開源、注重隱私的 Google Notebook LM 替代品。既然我們能夠掌控自己的研究工作流，為何還要將更多數據交給 Google？

在一個由人工智能主導的世界里，擁有思考🧠和獲取新知識💡的能力，不應是少數人的特權，也不應受限於單一的供應商。

**Open Notebook 賦予您以下能力:**
- 🔒 **掌控您的數據** - 確保您的研究私密且安全
- 🤖 **選擇您的 AI 模型** - 支持超過 16 家供應商，包括 OpenAI、Anthropic、Ollama、LM Studio 等
- 📚 **組織多模態內容** - 支持 PDF、視頻、音頻、網頁等多種格式
- 🎙️ **生成專業級播客** - 先進的多人對話播客生成功能
- 🔍 **智能搜索** - 對您的所有內容進行全文和向量搜索
- 💬 **結合上下文對話** - 由您的研究資料驅動的 AI 對話

訪問 [https: //www.open-notebook.ai](https://www.open-notebook.ai) 瞭解更多關於我們項目的信息

## 🆚 Open Notebook vs. Google Notebook LM

| 功能 | Open Notebook | Google Notebook LM | 優勢 |
|---------|---------------|--------------------|-----------|
| **隱私與控制** | 自主託管，數據歸您所有 | 僅限 Google Cloud | 完全的數據主權 |
| **AI 供應商選擇** | 超過 16 家供應商 (OpenAI, Anthropic, Ollama, LM Studio 等) | 僅限 Google 模型 | 靈活性與成本優化 |
| **播客發言人** | 1-4 位發言人，可自定義配置 | 僅限 2 位發言人 | 極高的靈活性 |
| **上下文控制** | 3 個精細層級 | 全有或全無 | 隱私與性能調優 |
| **內容轉換** | 自定義與內置轉換 | 選項有限 | 無限的處理能力 |
| **API 訪問** | 完整的 REST API | 無 API | 完全自動化 |
| **部署** | Docker、雲端或本地 | 僅限 Google 託管 | 隨處部署 |
| **引文** | 全面的來源引用 | 基礎的參考文獻 | 保障研究的完整性 |
| **定製化** | 開源，完全可定製 | 封閉系統 | 無限的可擴展性 |
| **成本** | 僅為 AI 使用付費 | 月度訂閱 + 使用費 | 透明且可控 |

**為何選擇 Open Notebook？**
- 🔒 **隱私優先**: 您的敏感研究將保持完全私密
- 💰 **成本控制**: 選擇更經濟的 AI 供應商，或通過 Ollama 在本地運行
- 🎙️ **更出色的播客**: 完全的腳本控制和多發言人靈活性，優於僅限 2 人的深度探討格式
- 🔧 **無限定製**: 根據需要修改、擴展和集成
- 🌐 **無供應商鎖定**: 隨時切換供應商、隨處部署、擁有您的數據

### 構建技術

[![Python][Python]][Python-url] [![SurrealDB][SurrealDB]][SurrealDB-url] [![LangChain][LangChain]][LangChain-url] [![Streamlit][Streamlit]][Streamlit-url]

## 🚀 快速入門

準備好試用 Open Notebook 了嗎？請選擇您偏好的方式:

### ⚡ 即時設置 (推薦)
```bash
# 為您的 Open Notebook 安裝創建一個新目錄
mkdir open-notebook
cd open-notebook

# 使用 Docker - 2 分鐘內即可開始
docker run -d \
  --name open-notebook \
  -p 8502:8502 -p 5055:5055 \
  -v ./notebook_data:/app/data \
  -v ./surreal_data:/mydata \
  -e OPENAI_API_KEY=your_key \
  lfnovo/open_notebook:latest-single
```

**將會創建什麼:**
```
open-notebook/
├── notebook_data/     # Your notebooks and research content
└── surreal_data/      # Database files
```

**訪問您的安裝實例:**
- **🖥️ 主界面**: http: //localhost: 8502 (Streamlit UI)
- **🔧 API 訪問**: http: //localhost: 5055 (REST API)
- **📚 API 文檔**: http: //localhost: 5055/docs (交互式 Swagger UI)

> **⚠️ 重要提示**:
> 1. **從專用文件夾運行**: 請在一個新的 `open-notebook` 文件夾內創建并運行，以確保您的數據卷被妥善組織
> 2. **數據卷持久化**: 數據卷 (`-v ./notebook_data:/app/data` 和 `-v ./surreal_data:/mydata`) 對於在容器重啓后保留數據至關重要。沒有它們，當容器停止時，您將丟失所有的筆記本和研究資料。

### 🛠️ 完整安裝
適用於開發或定製:
```bash
git clone https://github.com/lfnovo/open-notebook
cd open-notebook
make start-all
```

### 📖 需要幫助？
- **🤖 AI 安裝助手**: 我們構建了一個 [ 定製版 GPT 來幫助您安裝 Open Notebook](https://chatgpt.com/g/g-68776e2765b48191bd1bae3f30212631-open-notebook-installation-assistant) - 它將引導您完成每一步！
- ** 初次使用 Open Notebook？** 請從我們的 [ 入門指南 ](docs/getting-started/index.md) 開始
- ** 需要安裝幫助？** 請查閱我們的 [ 安裝指南 ](docs/getting-started/installation.md)
- ** 想看實際操作？** 請嘗試我們的 [ 快速入門教程 ](docs/getting-started/quick-start.md)

## 供應商支持矩陣

得益於 [Esperanto](https://github.com/lfnovo/esperanto) 庫，我們原生支持以下供應商！

| 供應商     | LLM 支持 | Embedding 支持 | 語音轉文本 | 文本轉語音 |
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

*支持 LM Studio 及任何與 OpenAI 兼容的端點

## ✨ 主要特性

### 核心能力
- **🔒 隱私優先**: 數據尽在掌握，無需依賴雲端
- **🎯 多筆記本管理**: 無縫管理多個研究項目
- **📚 通用內容支持**: 支持 PDF、視頻、音頻、網頁、Office 文檔等多種格式
- **🤖 多模型 AI 支持**: 支持超過 16 家提供商，包括 OpenAI、Anthropic、Ollama、Google、LM Studio 等
- **🎙️ 專業播客生成**: 通過“劇集配置”創建高級多主播播客
- **🔍 智能搜索**: 對您的所有內容進行全文和向量搜索
- **💬 上下文感知聊天**: 基於您的研究資料進行 AI 對話
- **📝 AI 輔助筆記**: 生成洞見或手動撰寫筆記

### 高級功能
- **⚡ 推理模型支持**: 全面支持 DeepSeek-R1、Qwen3 等思維模型
- **🔧 內容轉換**: 強大的可自定義操作，用於總結和提取洞見
- **🌐 全面的 REST API**: 通過完整的編程訪問實現自定義集成 <a href="http://localhost:5055/docs">![API 文檔 ](https://img.shields.io/badge/API-Documentation-blue?style=flat-square)</a>
- **🔐 可選密碼保護**: 通過身份驗證保護公開部署的安全
- **📊 細粒度上下文控制**: 精准選擇與 AI 模型共享的內容
- **📎 引用標註**: 獲取答案時附帶準確的來源引用

### 三欄式界面
1. **源文件**: 管理您的所有研究資料
2. **筆記**: 創建手動或 AI 生成的筆記
3. **聊天**: 以您的內容為上下文與 AI 進行對話

[![查看我們的播客示例](https://img.youtube.com/vi/D-760MlGwaI/0.jpg)](https://www.youtube.com/watch?v=D-760MlGwaI)

## 📚 文檔

### 入門指南
- **[📖 簡介](docs/getting-started/introduction.md)** - 瞭解 Open Notebook 的功能
- **[⚡ 快速入門](docs/getting-started/quick-start.md)** - 5 分鐘內完成設置并開始運行
- **[🔧 安裝](docs/getting-started/installation.md)** - 全面的安裝指南
- **[🎯 您的第一個筆記本](docs/getting-started/first-notebook.md)** - 分步教程

### 用戶指南
- **[📱 界面概覽](docs/user-guide/interface-overview.md)** - 瞭解界面佈局
- **[📚 筆記本](docs/user-guide/notebooks.md)** - 組織您的研究
- **[📄 源文件](docs/user-guide/sources.md)** - 管理內容類型
- **[📝 筆記](docs/user-guide/notes.md)** - 創建和管理筆記
- **[💬 聊天](docs/user-guide/chat.md)** - AI 對話
- **[🔍 搜索](docs/user-guide/search.md)** - 查找信息

### 高級主題
- **[🎙️ 播客生成](docs/features/podcasts.md)** - 創建專業播客
- **[🔧 內容轉換](docs/features/transformations.md)** - 自定義內容處理
- **[🤖 AI 模型](docs/features/ai-models.md)** - AI 模型配置
- **[🔧 REST API 參考](docs/development/api-reference.md)** - 完整的 API 文檔
- **[🔐 安全](docs/deployment/security.md)** - 密碼保護與隱私
- **[🚀 部署](docs/deployment/index.md)** - 適用於所有場景的完整部署指南

<p align="right">(<a href="#readme-top">返回頂部</a>)</p>

## 🗺️ 路線圖

### 即將推出的功能
- **React 前端**: 基於 React 的現代化前端，將取代 Streamlit
- **前端實時更新**: 實時 UI 更新，帶來更流暢的體驗
- **異步處理**: 通過異步內容處理提升 UI 響應速度
- **跨筆記本源文件**: 在不同項目間复用研究資料
- **書籤集成**: 連接您喜愛的書籤應用

### 近期完成 ✅
- **全面的 REST API**: 對所有功能提供完整的編程訪問
- **多模型支持**: 支持超過 16 家 AI 提供商，包括 OpenAI、Anthropic、Ollama、LM Studio
- **高級播客生成器**: 通過“劇集配置”創建專業的多主播播客
- **內容轉換**: 強大的可自定義操作，用於內容處理
- **增強的引用標註**: 改進佈局，更精細地控制來源引用
- **多聊天會話**: 在筆記本內管理不同的對話

查看 [ 公開的 issue](https://github.com/lfnovo/open-notebook/issues)，獲取擬議功能和已知問題的完整列表。

<p align="right">(<a href="#readme-top">返回頂部</a>)</p>


## 🤝 社區與貢獻

### 加入社區
- 💬 **[Discord 服務器](https://discord.gg/37XJPXfz2w)** - 獲取幫助、分享想法、并與其他用戶交流
- 🐛 **[GitHub Issues](https://github.com/lfnovo/open-notebook/issues)** - 報告錯誤和請求功能
- ⭐ **為本倉庫點亮 Star** - 表達您的支持，并幫助他人發現 Open Notebook

### 參與貢獻
我們歡迎各種貢獻！我們尤其需要以下方面的幫助:
- **前端開發**: 幫助構建基於 React 的現代化 UI（ 計劃取代當前的 Streamlit 界面 ）
- **測試與錯誤修復**: 讓 Open Notebook 更加穩健
- **功能開發**: 共同打造最酷的研究工具
- **文檔撰寫**: 改進指南和教程

**當前技術棧**: Python、FastAPI、SurrealDB、Streamlit
**未來路線圖**: React 前端、增強的實時更新

請參閱我們的 [ 貢獻指南 ](CONTRIBUTING.md)，瞭解如何開始的詳細信息。

<p align="right">(<a href="#readme-top">返回頂部</a>)</p>


## 📄 許可證

Open Notebook 採用 MIT 許可證。詳情請參閱 [LICENSE](LICENSE) 文件。

## 📞 聯繫方式

**Luis Novo** - [@lfnovo](https://twitter.com/lfnovo)

**社區支持**:
- 💬 [Discord 服務器 ](https://discord.gg/37XJPXfz2w) - 獲取幫助、分享想法、并與用戶交流
- 🐛 [GitHub Issues](https://github.com/lfnovo/open-notebook/issues) - 報告錯誤和請求功能
- 🌐 [ 網站 ](https://www.open-notebook.ai) - 瞭解關於本項目的更多信息

## 🙏 致謝

Open Notebook 的構建離不開衆多優秀的開源項目:

* **[Podcast Creator](https://github.com/lfnovo/podcast-creator)** - 高級播客生成功能
* **[Surreal Commands](https://github.com/lfnovo/surreal-commands)** - 後臺任務處理
* **[Content Core](https://github.com/lfnovo/content-core)** - 內容處理與管理
* **[Esperanto](https://github.com/lfnovo/esperanto)** - 多提供商 AI 模型抽象層
* **[Docling](https://github.com/docling-project/docling)** - 文檔處理與解析

<p align="right">(<a href="#readme-top">返回頂部</a>)</p>


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