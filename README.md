# RustのAtCoder用のリポジトリ

大元はrustの日本語ドキュメントサイトから取得。
https://github.com/rust-lang-ja/atcoder-proposal/
ローカルで実行する上で余分なファイルなどは削除している。

## 構成

```
rust_comp_pgm/
├── rust-toolchain.toml   # 全コンテスト共通 (1.74.0 + rust-analyzer コンポーネント)
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

## サンプルケースのテスト

サンプルを `abcxxx/samples/<問題>/` に `N.in` と `N.out` の組で置く（例: `abcxxx/samples/c/1.in`、`abcxxx/samples/c/1.out`）。
問題ページのサンプルをコピーして `pbpaste > abcxxx/samples/c/1.in` のように保存すると楽。

```
./test.sh abcxxx c    # ルートから
../test.sh c          # abcxxx/ の中から
```

- 各ケースを AC / WA / RE で表示し、WA なら期待する出力との差分を出す
- 行末の空白と末尾の空行は無視して比較する（小数の誤差許容には未対応）
- `.out` がないケースは実行結果だけ表示する
- `samples/` は AtCoder の問題文の一部なので `.gitignore` でコミット対象外にしている
