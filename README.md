# Vehicle Signal Shadow

Vehicle Signal Shadowは、車両のシグナルデータを管理するgRPCサービスです。  
VSS（Vehicle Signal Specification）に基づいて、車両の各種センサーやアクチュエーターのデータを取得・設定・購読することができます。  

## 機能

- **シグナル取得**: 指定されたパスのシグナル値を取得
- **シグナル設定**: シグナル値の設定（完全な置換または部分的な更新）
- **シグナル購読**: シグナル値の変更をリアルタイムで購読
- **シグナル購読解除**: 購読の停止

## ビルド

```bash
# サーバーのビルド
cargo build --release

# CLIクライアントのビルド
cd cli
cargo build --release
```

## 使用方法

### サーバーの起動

```bash
# 基本的な起動
cargo run -- --vss path/to/vss.json

# カスタム設定で起動
cargo run -- --vss path/to/vss.json --server-addr "0.0.0.0:50051" --log-level debug

# 環境変数を使用
export VSS_SERVER_ADDR="0.0.0.0:50051"
export VSS_LOG_LEVEL="debug"
cargo run -- --vss path/to/vss.json
```

### CLIクライアントの使用

```bash
# シグナル値の取得
./target/release/vehicle-signal-shadow-cli get "Vehicle.Speed"

# シグナル値の設定
./target/release/vehicle-signal-shadow-cli set --path "Vehicle.Speed" --value "60.5"

# 部分的な更新
./target/release/vehicle-signal-shadow-cli set --path "Vehicle.Speed" --value '{"value": 70.0, "capability": true}'

# シグナルの購読
./target/release/vehicle-signal-shadow-cli subscribe "Vehicle.Speed"
```

詳細な使用方法は [CLI README](cli/README.md) を参照してください。

## 設定

### コマンドライン引数

- `--vss`: VSS JSONファイルのパス（必須）
- `--server-addr`: サーバーのアドレス（デフォルト: "[::1]:50051"）
- `--log-level`: ログレベル（デフォルト: "info"）
- `--db-path`: データベースのパス（オプション、指定しない場合は一時ファイル）

### 環境変数

- `VSS_SERVER_ADDR`: サーバーのアドレス
- `VSS_LOG_LEVEL`: ログレベル
- `VSS_DB_PATH`: データベースのパス

## アーキテクチャ

```
src/
├── main.rs              # メインアプリケーション
├── config.rs            # 設定管理
├── error.rs             # エラー型定義
├── signal.rs            # シグナルデータ構造
├── vehicle_shadow.rs    # データベース操作
├── vss_json_loader.rs   # VSS JSONローダー
└── rpc/
    ├── mod.rs
    └── databroker_server.rs  # gRPCサーバー実装

cli/
├── Cargo.toml
├── build.rs
└── src/
    └── main.rs          # CLIクライアント

proto/
└── vehicle-shadow/      # Protocol Buffers定義
```

## 開発

### テストの実行

```bash
# 全テストの実行
cargo test

# 特定のテストの実行
cargo test test_signal_creation
```

### プロトコルバッファの再生成

```bash
cargo build
```

## ライセンス

このプロジェクトはMITライセンスの下で公開されています。 