# Vehicle Signal Shadow CLI

Vehicle Signal ShadowサービスのクライアントCLIツールです。

## ビルド

```bash
cd cli
cargo build --release
```

## 使用方法

### 基本的な使用方法

```bash
# ヘルプを表示
./target/release/vehicle-signal-shadow-cli --help

# サーバーを指定してコマンドを実行
./target/release/vehicle-signal-shadow-cli --server http://localhost:50051 <command>
```

### コマンド

#### 1. Get - シグナル値を取得

```bash
# 単一のシグナルを取得
./target/release/vehicle-signal-shadow-cli get "Vehicle.Speed"

# 複数のシグナルを取得
./target/release/vehicle-signal-shadow-cli get "Vehicle.Speed" "Vehicle.Engine.RPM" "Vehicle.Cabin.Temperature"
```

#### 2. Set - シグナル値を設定

```bash
# 単純な値を設定（従来の方法）
./target/release/vehicle-signal-shadow-cli set --path "Vehicle.Speed" --value "60.5"

# State全体を設定（新しい方法）
./target/release/vehicle-signal-shadow-cli set --path "Vehicle.Speed" --value '{"value": 60.5, "capability": true, "availability": true, "reserved": "updated"}'

# 部分的な更新（新しい機能）
# 値のみを更新
./target/release/vehicle-signal-shadow-cli set --path "Vehicle.Speed" --value '{"value": 70.0}'

# capabilityのみを更新
./target/release/vehicle-signal-shadow-cli set --path "Vehicle.Speed" --value '{"capability": false}'

# 複数のフィールドを同時に更新
./target/release/vehicle-signal-shadow-cli set --path "Vehicle.Speed" --value '{"value": 80.0, "availability": true}'

# 値のみを設定（capability, availability, reservedは更新されない）
./target/release/vehicle-signal-shadow-cli set --path "Vehicle.Lights.Headlight.IsOn" --value "true"

# 文字列を設定
./target/release/vehicle-signal-shadow-cli set --path "Vehicle.Cabin.Driver.Name" --value "\"John Doe\""

# 配列を設定
./target/release/vehicle-signal-shadow-cli set --path "Vehicle.Sensors.Temperature" --value "[25.5, 26.0, 24.8]"
```

#### 3. Subscribe - シグナル変更を購読

```bash
# 単一のシグナルを購読
./target/release/vehicle-signal-shadow-cli subscribe "Vehicle.Speed"

# 複数のシグナルを購読
./target/release/vehicle-signal-shadow-cli subscribe "Vehicle.Speed" "Vehicle.Engine.RPM"

# Ctrl+Cで購読を停止
```

#### 4. Unsubscribe - シグナル購読を解除

```bash
# シグナル購読を解除
./target/release/vehicle-signal-shadow-cli unsubscribe "Vehicle.Speed" "Vehicle.Engine.RPM"
```

## サポートされている値の型

### 基本型
- `bool`: ブール値 (例: `true`, `false`)
- `string`: 文字列 (例: `"Hello World"`)
- `number`: 数値 (例: `42`, `3.14`)

### 配列型
- `bool[]`: ブール配列 (例: `[true, false, true]`)
- `string[]`: 文字列配列 (例: `["a", "b", "c"]`)
- `number[]`: 数値配列 (例: `[1, 2, 3, 4]`)

## 例

### 車両の速度を監視

```bash
# 速度を購読
./target/release/vehicle-signal-shadow-cli subscribe "Vehicle.Speed"
```

### エンジンRPMを設定

```bash
# RPMを2000に設定
./target/release/vehicle-signal-shadow-cli set --path "Vehicle.Engine.RPM" --value "2000"
```

### 複数のシグナルを一度に取得

```bash
# 車両の主要な情報を取得
./target/release/vehicle-signal-shadow-cli get \
  "Vehicle.Speed" \
  "Vehicle.Engine.RPM" \
  "Vehicle.Cabin.Temperature" \
  "Vehicle.Lights.Headlight.IsOn"
```

## エラーハンドリング

CLIは以下のエラーを適切に処理します：

- サーバー接続エラー
- 無効なシグナルパス
- 無効な値の型
- ネットワークタイムアウト

エラーが発生した場合は、適切なエラーメッセージが表示されます。 