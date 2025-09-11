![Volo](https://github.com/cloudwego/volo/raw/main/.github/assets/logo.png?sanitize=true)

[![Crates.io](https://img.shields.io/crates/v/volo)](https://crates.io/crates/volo)
[![Documentation](https://docs.rs/volo/badge.svg)](https://docs.rs/volo)
[![Website](https://img.shields.io/website?up_message=cloudwego&url=https%3A%2F%2Fwww.cloudwego.io%2F)](https://www.cloudwego.io/)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/cloudwego/volo)
[![License](https://img.shields.io/crates/l/volo)](#license)
[![Build Status][actions-badge]][actions-url]

[actions-badge]: https://github.com/cloudwego/volo/actions/workflows/ci.yaml/badge.svg
[actions-url]: https://github.com/cloudwego/volo/actions

[English](README.md) | 中文 | [日本語](README-ja.md)

Volo 是字節跳動服務框架團隊研發的 **高性能**、**可擴展性強** 的 Rust RPC 框架，使用了 Rust 最新的 AFIT 和 RPITIT 特性。

Volo 使用 [`Motore`][motore] 作為其中間件抽象層, Motore 基於 AFIT 和 RPITIT 設計。

## 概覽

### Crates

Volo 主要包含 6 個 crate 庫:

1. [`volo`][volo] - 包含框架的通用組件。
2. [`volo-thrift`][volo-thrift] - 提供 **thrift** RPC 消息協議支持。
3. [`volo-grpc`][volo-grpc] - 提供 **gRPC** RPC 消息協議支持。
4. [`volo-build`][volo-build] - 通過 **thrift** 或 **protobuf** 文件生成 rust 代碼。
5. [`volo-cli`][volo-cli] - 命令行工具，基於 thrift 和 protobuf 的 IDL 生成 項目腳手架。
6. [`volo-macros`][volo-macros] - 框架的中間件抽象層。

### 特點

#### 使用 AFIT 和 RPITIT 特性

Volo 使用 [`Motore`][motore] 作為其中間件抽象層, Motore 基於 AFIT 和 RPITIT 設計。

通過 RPITIT，我們可以避免很多不必要的 Box 內存分配，以及提升易用性，給用戶提供更友好的編程接口和更符合人體工程學的編程范式。

#### 高性能

Rust 以高性能和安全著稱，我們在設計和實現過程中也時刻以高性能作為我們的目標，尽可能降低每一處的開銷，提升每一處實現的性能。

首先要說明，**和 Go 的框架對比性能是極不公平的**，因此我們不會著重比較 Volo 和 Kitex 的性能，並且我們給出的數據僅能作為參考，希望大家能夠客觀看待；同時，由於在開源社區并沒有找到另一款成熟的 Rust 語言的 Async 版本 Thrift RPC 框架，而且性能對比總是容易引戰，因此我們希望尽可能弱化性能數據的對比，僅會公佈我們自己極限 QPS 的數據。

在和 Kitex 相同的測試條件（限制 4C）下，Volo 極限 QPS 為 35W；同時，我們內部正在驗證基於 [Monoio](https://github.com/bytedance/monoio)（CloudWeGo 開源的 Rust Async Runtime）的版本，極限 QPS 可以達到 44W。

從我們線上業務的火焰圖來看，得益於 Rust 的靜態分發和優秀的編譯優化，框架部分的開銷基本可以忽略不計（不包含 syscall 開銷）。

#### 易用性好

~~Rust 以難學難用而聞名~~，我們希望尽可能降低用戶使用 Volo 框架以及使用 Rust 語言編寫微服務的難度，提供最符合人體工程學和直覺的編碼體驗。因此，我們把易用性作為我們最重要的目標之一。

比如，我們提供了 volo 命令行工具，用於初始化項目以及管理 idl；同時，我們將 thrift 及 gRPC 拆分為兩個獨立（但共用一些組件）的框架，以提供最符合不同協議語義的編程范式及接口。

我們還提供了`#[service]`宏（可以理解為不需要 `Box` 的 `async_trait`）來使得用戶可以無心理負擔地使用異步來編寫 `Service` 中間件。

#### 擴展性強

收益於 Rust 強大的表達和抽象能力，通過靈活的中間件 Service 抽象，開發者可以以非常統一的形式，對 RPC 元信息、請求和響應做處理。

比如，服務發現、負載均衡等服務治理功能，都可以以 Service 形式進行實現，而不需要獨立實現 Trait。

相關的擴展，我們會放在 [volo-rs](https://github.com/volo-rs) 組織下，也歡迎大家貢獻自己的擴展到 volo-rs。

查看 [guide](https://www.cloudwego.io/zh/docs/volo/guide/) 獲取更多信息。

## 相關教程

Volo-Thrift: https://www.cloudwego.io/zh/docs/volo/volo-thrift/getting-started/

Volo-gRPC: https://www.cloudwego.io/zh/docs/volo/volo-grpc/getting-started/

## 示例

參考[Examples](examples).

## 相關生態

- [Volo-rs][volo-rs]: Volo 的相關生態，包含了 Volo 的許多組件
- [Motore][motore]: Volo 參考 Tower 設計的，使用了 AFIT 和 RPITIT 的 middleware 抽象層
- [Pilota][pilota]: Volo 使用的 Thrift 與 Protobuf 編譯器及編解碼的純 Rust 實現（不依賴 protoc）
- [Metainfo][metainfo]: Volo 用於進行元信息透傳的組件，期望定義一套元信息透傳的標準

## 開發路線圖

點擊 [ROADMAP.md](https://github.com/cloudwego/volo/blob/main/ROADMAP.md) 獲取更多信息。

## 如何貢獻

點擊 [CONTRIBUTING.md](https://github.com/cloudwego/volo/blob/main/CONTRIBUTING.md) 獲取更多信息。

## 開源許可

Volo 使用 MIT license 和 the Apache License (Version 2.0) 雙重許可證。

點擊 [LICENSE-MIT](https://github.com/cloudwego/volo/blob/main/LICENSE-MIT) 和 [LICENSE-APACHE](https://github.com/cloudwego/volo/blob/main/LICENSE-APACHE) 查看更多細節。

## 鳴謝

我們使用了一些第三方組件, 在此感謝他們的付出

點擊 [CREDITS.md](https://github.com/cloudwego/volo/blob/main/CREDITS.md) 查看完整的名單。

## 社區

- Email: [volo@cloudwego.io](mailto:volo@cloudwego.io)
- 如何成為 member: [COMMUNITY MEMBERSHIP](https://github.com/cloudwego/community/blob/main/COMMUNITY_MEMBERSHIP.md)
- Issues: [Issues](https://github.com/cloudwego/volo/issues)
- 飛書用戶群: 通過 [Feishu](https://www.feishu.cn/) app 掃描下方的二維碼 或者 [點擊連接](https://applink.feishu.cn/client/chat/chatter/add_by_link?link_token=b34v5470-8e4d-4c7d-bf50-8b2917af026b) 加入我們的 CloudWeGo Volo 用戶群。

  <img src="https://github.com/cloudwego/volo/raw/main/.github/assets/volo-feishu-user-group.png" alt="Volo user group" width="50%" height="50%" />

[volo-rs]: https://github.com/volo-rs
[motore]: https://github.com/cloudwego/motore
[pilota]: https://github.com/cloudwego/pilota
[metainfo]: https://github.com/cloudwego/metainfo
[volo]: https://docs.rs/volo
[volo-thrift]: https://docs.rs/volo-thrift
[volo-grpc]: https://docs.rs/volo-grpc
[volo-build]: https://docs.rs/volo-build
[volo-cli]: https://crates.io/crates/volo-cli
[volo-macros]: https://docs.rs/volo-macros
[examples]: https://github.com/cloudwego/volo/tree/main/examples