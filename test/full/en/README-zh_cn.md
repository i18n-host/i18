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

Volois a **high-performance** and **highly scalable** Rust RPC framework developed by the ByteDance Service Framework team. It leverages Rust's latest AFIT and RPITIT features.

VoloIt uses [`Motore`][motore] as its middleware abstraction layer, which is designed based on AFIT and RPITIT.

## Overview

### Crates

Volo mainly consists of 6 crates:

1. [`volo`][volo] - Contains the common components of the framework.
2. [`volo-thrift`][volo-thrift] - Provides support for the **Thrift** RPC protocol.
3. [`volo-grpc`][volo-grpc] - Provides support for the **gRPC** RPC protocol.
4. [`volo-build`][volo-build] - Generates Rust code from **Thrift** or **Protobuf** files.
5. [`volo-cli`][volo-cli] - A command-line tool to generate project scaffolding from Thrift and Protobuf IDLs.
6. [`volo-macros`][volo-macros] - Provides procedural macros for the framework.

### Features

#### Uses AFIT and RPITIT

VoloIt uses [`Motore`][motore] as its middleware abstraction layer, which is designed based on AFIT and RPITIT.

With RPITIT, we can avoid many unnecessary Box allocations, improve usability, and provide users with a more friendly programming interface and a more ergonomic programming paradigm.

#### High Performance

RustRenowned for its high performance and safety, Rust guides our design and implementation. We consistently aim for high performance, striving to minimize overhead and optimize every part of the implementation.

First, it must be stated that **comparing performance with Go frameworks is extremely unfair**. Therefore, we will not focus on comparing the performance of Volo and Kitex. The data we provide is for reference only, and we hope everyone can view it objectively. At the same time, since we have not found another mature async Thrift RPC framework in the Rust open-source community, and performance comparisons are always prone to controversy, we wish to downplay performance data comparisons and will only release our own peak QPS data.

Under the same test conditions as Kitex (limited to 4C), Volo's peak QPS is 350k. Meanwhile, we are internally verifying a version based on [Monoio](https://github.com/bytedance/monoio) (an open-source Rust Async Runtime by CloudWeGo), which can achieve a peak QPS of 440k.

From the flame graphs of our online services, thanks to Rust's static dispatch and excellent compiler optimizations, the framework's overhead is almost negligible (excluding syscall overhead).

#### Ease of Use

~~Rust is notorious for being difficult to learn and use~~. We hope to lower the barrier for users to adopt the Volo framework and write microservices in Rust, providing the most ergonomic and intuitive coding experience. Therefore, we have made ease of use one of our most important goals.

For example, we provide the volo command-line tool for initializing projects and managing IDLs. We have also separated Thrift and gRPC into two independent frameworks (though they share some components) to offer programming paradigms and interfaces that best fit the semantics of each protocol.

We also provide the `#[service]` macro (which can be thought of as a `Box`-less `async_trait`) to allow users to write async `Service` middleware without any mental burden.

#### High Extensibility

Benefiting from Rust's powerful expression and abstraction capabilities, developers can process RPC metadata, requests, and responses in a very unified way through a flexible middleware Service abstraction.

For example, service governance features like service discovery and load balancing can all be implemented as a Service, without needing to implement a separate Trait.

Related extensions will be placed under the [volo-rs](https://github.com/volo-rs) organization. We welcome everyone to contribute their own extensions to volo-rs.

See the [guide](https://www.cloudwego.io/zh/docs/volo/guide/) for more information.

## Related Tutorials

Volo-Thrift: https://www.cloudwego.io/zh/docs/volo/volo-thrift/getting-started/

Volo-gRPC: https: //www.cloudwego.io/zh/docs/volo/volo-grpc/getting-started/

## Examples

See [Examples](examples).

## Related Ecosystem

- [Volo-rs][volo-rs]: The ecosystem for Volo, which includes many of its components
- [Motore][motore]: A middleware abstraction layer inspired by Tower, using AFIT and RPITIT
- [Pilota][pilota]: A pure Rust implementation of the Thrift and Protobuf compiler and codec used by Volo (with no dependency on protoc)
- [Metainfo][metainfo]: A component for transparent metainfo transmission, aiming to define a standard for it

## Roadmap

See [ROADMAP.md](https://github.com/cloudwego/volo/blob/main/ROADMAP.md) for more information.

## Contributing

See [CONTRIBUTING.md](https://github.com/cloudwego/volo/blob/main/CONTRIBUTING.md) for more information.

## License

VoloLicensed under both the MIT license and the Apache License, Version 2.0.

See [LICENSE-MIT](https://github.com/cloudwego/volo/blob/main/LICENSE-MIT) and [LICENSE-APACHE](https://github.com/cloudwego/volo/blob/main/LICENSE-APACHE) for details.

## Credits

We would like to thank the contributors of the third-party components we use

See [CREDITS.md](https://github.com/cloudwego/volo/blob/main/CREDITS.md) for a complete list.

## Community

- Email:[volo@cloudwego.io](mailto:volo@cloudwego.io)
- How to become a member:  [COMMUNITY MEMBERSHIP](https://github.com/cloudwego/community/blob/main/COMMUNITY_MEMBERSHIP.md)
- Issues:[Issues](https://github.com/cloudwego/volo/issues)
- Feishu Group:  Scan the QR code below with the [Feishu](https://www.feishu.cn/) app or [click the link](https://applink.feishu.cn/client/chat/chatter/add_by_link?link_token=b34v5470-8e4d-4c7d-bf50-8b2917af026b) to join our CloudWeGo Volo user group.

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