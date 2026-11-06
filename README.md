# RustのAtCoder用のリポジトリ

大元はrustの日本語ドキュメントサイトから取得。
https://github.com/rust-lang-ja/atcoder-proposal/
ローカルで実行する上で余分なファイルなどは削除している。

## 構成

```
rust_comp_pgm/
├── rust-toolchain        # 全コンテスト共通 (1.74.0: rust-analyzer 対応のため)
├── .cargo/config.toml    # target/ をルートで共有 → 依存クレートのビルドは初回のみ
├── .vscode/launch.json   # 開いている問題ファイルをデバッグ実行
├── base/                 # テンプレート
└── abcXXX/ など          # コンテストごとのディレクトリ
    ├── Cargo.toml / Cargo.lock
    └── src/bin/{a..f}.rs # 各問題の解答
```

## コンテスト準備

baseディレクトリをコピーする
`cp -r base abcxxx`

## プログラム実行方法

VS Code ではリポジトリのルートを開く。

- デバッグ: 解きたい問題のファイル（例: `abcxxx/src/bin/c.rs`）を開いて F5（"Debug current problem"）。標準入力はターミナルに入力する
- ターミナル: `cd abcxxx && cargo run --bin c`
