<a id="readme-top"></a>

<!-- [![Contributors][contributors-shield]][contributors-url] -->
[![フォ ー ク][forks-shield]][forks-url]
[![スタ ー を付けたユ ー ザ ー][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT ライセンス][license-shield]][license-url]
<!-- [![LinkedIn][linkedin-shield]][linkedin-url] -->


<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/lfnovo/open-notebook">
    <img src="docs/assets/hero.svg" alt="Logo">
  </a>

  <h3 align="center">Open Notebook</h3>

  <p align="center">
    Google Notebook LM に代わる、プライバシ ー を重視したオ ー プンソ ー スのツ ー ルです
    <br /><strong>ヘルプ、ワ ー クフロ ー のアイデア共有、機能提案は、<a href="https://discord.gg/37XJPXfz2w">Discord サ ー バ ー</a> にご参加ください！</strong>
    <br />
    <a href="https://www.open-notebook.ai"><strong>公式サイトはこちら »</strong></a>
    <br />
    <br />
    <a href="docs/getting-started/index.md">📚 利用を開始する</a>
    ·
    <a href="docs/user-guide/index.md">📖 ユ ー ザ ー ガイド</a>
    ·
    <a href="docs/features/index.md">✨ 機能</a>
    ·
    <a href="docs/deployment/index.md">🚀 デプロイ</a>
  </p>
</div>

## 📢 Open Notebook は現在、活発に開発が進められています

> Open Notebook は現在、活発に開発が進められています！私たちは毎週、迅速な改善を重ねています。このエキサイティングな段階において、皆様からのフィ ー ドバックは私にとって非常に貴重であり、この素晴らしいツ ー ルを改善し、構築し続けるためのモチベ ー ションになります。このプロジェクトが役に立つと思われましたら、ぜひスタ ー を付けてください。また、ご質問やご提案がありましたら、お気軽にご連絡ください。皆様がこのツ ー ルをどのように活用し、どのようなアイデアをプロジェクトにもたらしてくださるか、楽しみにしています！一緒に素晴らしいものを作り上げましょう！ 🚀

## このプロジェクトについて

![新しいノ ー トブック](docs/assets/asset_list.png)

Google Notebook LM に代わる、プライバシ ー を重視したオ ー プンソ ー スのツ ー ルです。自分自身の研究ワ ー クフロ ー を管理できるのに、なぜ Google にこれ以上デ ー タを提供する必要があるのでしょうか？

人工知能が主流となる世界において、思考し🧠、新たな知識を得る💡能力は、一部の特権であってはならず、単一のプロバイダ ー に限定されるべきスキルでもありません。

**Open Notebook でできること:**
- 🔒 **デ ー タを自分で管理** - 研究のプライバシ ー とセキュリティを保護
- 🤖 **AI モデルを自由に選択** - OpenAI、Anthropic、Ollama、LM Studio など 16 以上のプロバイダ ー をサポ ー ト
- 📚 **多様な形式のコンテンツを整理** - PDF、動画、音声、ウェブペ ー ジなど
- 🎙️ **プロ品質のポッドキャストを生成** - 高度な複数話者によるポッドキャスト生成
- 🔍 **インテリジェントな検索** - すべてのコンテンツを対象とした全文検索とベクトル検索
- 💬 **文脈に基づいたチャット** - あなたの研究内容に基づいた AI との対話

プロジェクトの詳細は [https: //www.open-notebook.ai](https://www.open-notebook.ai) をご覧ください

## 🆚 Open Notebook vs Google Notebook LM

| 機能 | Open Notebook | Google Notebook LM | 優位点 |
|---------|---------------|--------------------|-----------|
| **プライバシ ー と管理権** | セルフホスト、あなたのデ ー タ | Google Cloud のみ | 完全なデ ー タ主権 |
| **AI プロバイダ ー の選択肢** | 16 以上のプロバイダ ー（OpenAI、Anthropic、Ollama、LM Studio など ） | Google のモデルのみ | 柔軟性とコスト最適化 |
| **ポッドキャストの話者** | カスタムプロファイルを持つ 1～4 人の話者 | 2 人の話者のみ | 非常に高い柔軟性 |
| **コンテキスト制御** | 3 段階のきめ細かなレベル | 全選択か全解除のみ | プライバシ ー とパフォ ー マンスの調整 |
| **コンテンツ変換** | カスタムおよび組み込み | 限定的なオプション | 無限の処理能力 |
| **API アクセス** | 完全な REST API | API なし | 完全な自動化 |
| **デプロイ** | Docker、クラウド、またはロ ー カル | Google ホストのみ | どこにでもデプロイ可能 |
| **引用** | 出典付きで包括的に表示 | 基本的な参考文献 | 研究の完全性 |
| **カスタマイズ** | オ ー プンソ ー ス、完全にカスタマイズ可能 | クロ ー ズドシステム | 無限の拡張性 |
| **コスト** | AI 利用料のみ | 月額サブスクリプション ＋ 利用料 | 透明性が高く、管理可能 |

**Open Notebook を選ぶ理由**
- 🔒 **プライバシ ー 第一**: 機密性の高い研究を完全にプライベ ー トに保ちます
- 💰 **コスト管理**: より安価な AI プロバイダ ー を選択、または Ollama でロ ー カル実行
- 🎙️ **より良いポッドキャスト**: 完全なスクリプト制御と複数話者の柔軟性 vs 限定的な 2 話者形式
- 🔧 **無限のカスタマイズ**: 必要に応じて変更、拡張、統合が可能
- 🌐 **ベンダ ー ロックインなし**: プロバイダ ー の切り替え、どこへでもデプロイ、デ ー タの所有

### 使用技術

[![Python][Python]][Python-url] [![SurrealDB][SurrealDB]][SurrealDB-url] [![LangChain][LangChain]][LangChain-url] [![Streamlit][Streamlit]][Streamlit-url]

## 🚀 クイックスタ ー ト

Open Notebook を試す準備はできましたか？お好みの方法を選択してください:

### ⚡ クイックセットアップ （ 推奨 ）
```bash
# Open Notebook のインスト ー ル用に新しいディレクトリを作成します
mkdir open-notebook
cd open-notebook

# Docker を使用 - 2 分で開始
docker run -d \
  --name open-notebook \
  -p 8502:8502 -p 5055:5055 \
  -v ./notebook_data:/app/data \
  -v ./surreal_data:/mydata \
  -e OPENAI_API_KEY=your_key \
  lfnovo/open_notebook:latest-single
```

**作成されるもの:**
```
open-notebook/
├── notebook_data/     # Your notebooks and research content
└── surreal_data/      # Database files
```

**インスト ー ルへのアクセス:**
- **🖥️ メインインタ ー フェ ー ス**: http: //localhost: 8502 (Streamlit UI)
- **🔧 API アクセス**: http: //localhost: 5055 (REST API)
- **📚 API ドキュメント**: http: //localhost: 5055/docs (インタラクティブなSwagger UI)

> **⚠️ 重要**:
> 1. **専用フォルダから実行してください**: デ ー タボリュ ー ムが適切に整理されるよう、新しい `open-notebook` フォルダを作成し、その中で実行してください
> 2. **ボリュ ー ムの永続化**: ボリュ ー ム （`-v ./notebook_data:/app/data` および `-v ./surreal_data:/mydata`） は、コンテナを再起動してもデ ー タを永続化させるために不可欠です。これらがないと、コンテナ停止時にすべてのノ ー トブックと研究内容が失われます。

### 🛠️ フルインスト ー ル
開発またはカスタマイズ向け:
```bash
git clone https://github.com/lfnovo/open-notebook
cd open-notebook
make start-all
```

### 📖 ヘルプ
- **🤖 AI インスト ー ルアシスタント**: Open Notebook のインスト ー ルを支援するために構築された [ カスタム GPT](https://chatgpt.com/g/g-68776e2765b48191bd1bae3f30212631-open-notebook-installation-assistant) があります。各ステップをガイドします！
- **Open Notebook は初めてですか？** まずは [ スタ ー トガイド ](docs/getting-started/index.md) から始めましょう
- ** インスト ー ルでお困りですか？** [ インスト ー ルガイド ](docs/getting-started/installation.md) をご確認ください
- ** 実際の動作を見てみたいですか？** [ クイックスタ ー トチュ ー トリアル ](docs/getting-started/quick-start.md) をお試しください

## プロバイダ ー サポ ー トマトリクス

[Esperanto](https://github.com/lfnovo/esperanto) ライブラリのおかげで、これらのプロバイダ ー をすぐに利用できます！

| プロバイダ ー     | LLM サポ ー ト | 埋め込みサポ ー ト | 音声認識 (Speech-to-Text) | 音声合成 (Text-to-Speech) |
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
| OpenAI 互換* | ✅          | ❌               | ❌             | ❌             |

*LM Studio および任意の OpenAI 互換エンドポイントをサポ ー ト

## ✨ 主な機能

### コア機能
- **🔒 プライバシ ー 第一**: デ ー タは常にあなたの管理下にあり、クラウドへの依存はありません
- **🎯 マルチノ ー トブック構成**: 複数の研究プロジェクトをシ ー ムレスに管理します
- **📚 ユニバ ー サルコンテンツサポ ー ト**: PDF、動画、音声、Web ペ ー ジ、Office ドキュメントなどに対応
- **🤖 マルチモデル AI サポ ー ト**: OpenAI、Anthropic、Ollama、Google、LM Studio など 16 以上のプロバイダ ー に対応
- **🎙️ プロ品質のポッドキャスト生成**: エピソ ー ドプロファイルを使用した高度なマルチスピ ー カ ー ポッドキャスト
- **🔍 インテリジェント検索**: すべてのコンテンツを対象とした全文検索とベクトル検索
- **💬 コンテキスト認識チャット**: あなたの研究資料に基づいた AI との対話
- **📝 AI アシストノ ー ト**: インサイトを生成したり、手動でメモを作成したりできます

### 高度な機能
- **⚡ 推論モデルのサポ ー ト**: DeepSeek-R1 や Qwen3 のような思考モデルを完全にサポ ー ト
- **🔧 コンテンツ変換**: インサイトの要約 ・ 抽出を行う、カスタマイズ可能な強力なアクション
- **🌐 包括的な REST API**: カスタム統合のための完全なプログラマティックアクセス。<a href="http://localhost:5055/docs">![API ドキュメント ](https://img.shields.io/badge/API-Documentation-blue?style=flat-square)</a>
- **🔐 オプションのパスワ ー ド保護**: 認証機能で公開デプロイメントを安全に保護します
- **📊 きめ細かなコンテキスト制御**: AI モデルと共有する情報を正確に選択できます
- **📎 引用表示**: 適切な出典元を引用付きで回答を得られます

### 3 カラムインタ ー フェ ー ス
1. **ソ ー ス**: すべての研究資料を管理します
2. **ノ ー ト**: 手動または AI によるノ ー トを作成します
3. **チャット**: あなたのコンテンツをコンテキストとして AI と対話します

[![ポッドキャストのサンプルをチェック](https://img.youtube.com/vi/D-760MlGwaI/0.jpg)](https://www.youtube.com/watch?v=D-760MlGwaI)

## 📚 ドキュメンテ ー ション

### はじめに
- **[📖 イントロダクション](docs/getting-started/introduction.md)** - Open Notebook で何ができるかを学びます
- **[⚡ クイックスタ ー ト](docs/getting-started/quick-start.md)** - 5 分でセットアップして使い始めましょう
- **[🔧 インスト ー ル](docs/getting-started/installation.md)** - 包括的なセットアップガイド
- **[🎯 最初のノ ー トブック](docs/getting-started/first-notebook.md)** - ステップバイステップのチュ ー トリアル

### ユ ー ザ ー ガイド
- **[📱 インタ ー フェ ー ス概要](docs/user-guide/interface-overview.md)** - レイアウトを理解します
- **[📚 ノ ー トブック](docs/user-guide/notebooks.md)** - 研究を整理します
- **[📄 ソ ー ス](docs/user-guide/sources.md)** - コンテンツタイプを管理します
- **[📝 ノ ー ト](docs/user-guide/notes.md)** - ノ ー トを作成 ・ 管理します
- **[💬 チャット](docs/user-guide/chat.md)** - AI との対話
- **[🔍 検索](docs/user-guide/search.md)** - 情報を見つけます

### 高度なトピック
- **[🎙️ ポッドキャスト生成](docs/features/podcasts.md)** - プロ品質のポッドキャストを作成します
- **[🔧 コンテンツ変換](docs/features/transformations.md)** - コンテンツ処理をカスタマイズします
- **[🤖 AI モデル](docs/features/ai-models.md)** - AI モデルの設定
- **[🔧 REST API リファレンス](docs/development/api-reference.md)** - 完全な API ドキュメント
- **[🔐 セキュリティ](docs/deployment/security.md)** - パスワ ー ド保護とプライバシ ー
- **[🚀 デプロイ](docs/deployment/index.md)** - あらゆるシナリオに対応する完全なデプロイガイド

<p align="right">(<a href="#readme-top">トップへ戻る</a>)</p>

## 🗺️ ロ ー ドマップ

### 今後の機能
- **React フロントエンド**: Streamlit を置き換える、モダンな React ベ ー スのフロントエンド
- **ライブフロントエンド更新**: リアルタイムの UI 更新で、よりスム ー ズな体験を実現
- **非同期処理**: 非同期のコンテンツ処理による高速な UI
- **クロスノ ー トブックソ ー ス**: 研究資料をプロジェクト間で再利用
- **ブックマ ー ク連携**: お気に入りのブックマ ー クアプリと連携

### 最近完了した機能 ✅
- **包括的な REST API**: すべての機能への完全なプログラマティックアクセス
- **マルチモデルサポ ー ト**: OpenAI、Anthropic、Ollama、LM Studio など 16 以上の AI プロバイダ ー に対応
- **高度なポッドキャストジェネレ ー タ ー**: エピソ ー ドプロファイルを使用したプロ品質のマルチスピ ー カ ー ポッドキャスト
- **コンテンツ変換**: コンテンツ処理のための、カスタマイズ可能な強力なアクション
- **引用機能の強化**: 引用元のレイアウト改善と、よりきめ細かな制御
- **複数チャットセッション**: ノ ー トブック内で異なる会話を管理

提案された機能と既知の問題の全リストは、[ 公開されている issue](https://github.com/lfnovo/open-notebook/issues) をご覧ください。

<p align="right">(<a href="#readme-top">トップへ戻る</a>)</p>


## 🤝 コミュニティと貢献

### コミュニティに参加
- 💬 **[Discord サ ー バ ー](https://discord.gg/37XJPXfz2w)** - ヘルプを得たり、アイデアを共有したり、他のユ ー ザ ー と繋がりましょう
- 🐛 **[GitHub Issues](https://github.com/lfnovo/open-notebook/issues)** - バグの報告や機能のリクエスト
- ⭐ **このリポジトリにスタ ー を付ける** - Open Notebook を応援し、他のユ ー ザ ー が見つけやすくしましょう

### コントリビュ ー ション
コントリビュ ー ションを歓迎します！特に、以下の分野でご協力いただける方を募集しています:
- **フロントエンド開発**: モダンな React ベ ー スの UI 構築 （ 現在の Streamlit インタ ー フェ ー スを置き換える予定です ）
- **テストとバグ修正**: Open Notebook をより堅牢にするための協力
- **機能開発**: 最高の研究ツ ー ルを一緒に作り上げましょう
- **ドキュメンテ ー ション**: ガイドやチュ ー トリアルの改善

**現在の技術スタック**: Python, FastAPI, SurrealDB, Streamlit
**今後のロ ー ドマップ**: React フロントエンド、リアルタイム更新の強化

貢献を始めるための詳細については、[ コントリビュ ー ションガイド ](CONTRIBUTING.md) をご覧ください。

<p align="right">(<a href="#readme-top">トップへ戻る</a>)</p>


## 📄 ライセンス

Open Notebook は MIT ライセンスです。詳細については、[LICENSE](LICENSE) ファイルをご覧ください。

## 📞 連絡先

**Luis Novo** - [@lfnovo](https://twitter.com/lfnovo)

**コミュニティサポ ー ト**:
- 💬 [Discord サ ー バ ー](https://discord.gg/37XJPXfz2w) - ヘルプ、アイデアの共有、ユ ー ザ ー との交流の場
- 🐛 [GitHub Issues](https://github.com/lfnovo/open-notebook/issues) - バグ報告と機能リクエスト
- 🌐 [ ウェブサイト ](https://www.open-notebook.ai) - プロジェクトの詳細情報

## 🙏 謝辞

Open Notebook は、素晴らしいオ ー プンソ ー スプロジェクトの功績の上に成り立っています:

* **[Podcast Creator](https://github.com/lfnovo/podcast-creator)** - 高度なポッドキャスト生成機能
* **[Surreal Commands](https://github.com/lfnovo/surreal-commands)** - バックグラウンドでのジョブ処理
* **[Content Core](https://github.com/lfnovo/content-core)** - コンテンツの処理と管理
* **[Esperanto](https://github.com/lfnovo/esperanto)** - マルチプロバイダ ーAI モデルの抽象化
* **[Docling](https://github.com/docling-project/docling)** - ドキュメントの処理と解析

<p align="right">(<a href="#readme-top">トップへ戻る</a>)</p>


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