# Seeds - 初期データ管理システム

このディレクトリには、データベース初期化用のシードデータが含まれています。

## 使用方法

### 基本的な使用方法

```bash
# 開発環境用データを投入
cargo run --bin seeds -- --env development

# 特定のテーブルのみ投入
cargo run --bin seeds -- --env development --table departments

# ドライランモード（実際には投入せず確認のみ）
cargo run --bin seeds -- --env development --dry-run

# 既存データをリセットして再投入
cargo run --bin seeds -- --env development --reset
```

### 環境について

- **development**: 開発環境用のテストデータ
- **test**: 自動テスト用のデータ
- **production**: 本番環境用の最小限のデータ

## データファイル構造

### ディレクトリ構成

```tree
seeds/
├── data/
│   ├── development/     # 開発環境用
│   ├── test/           # テスト環境用
│   └── production/     # 本番環境用
└── templates/          # データテンプレート
```

### JSONファイル形式

```json
{
  "version": "1.0",
  "environment": "development",
  "description": "部署マスタデータ",
  "dependencies": ["employees"],
  "data": [
    {
      "code": "DEV",
      "name": "開発部",
      "description": "システム開発とメンテナンス"
    }
  ]
}
```

### データ投入順序

1. employees（従業員）
2. departments（部署）
3. document_types（文書種別）
4. document_number_generation_rules（採番ルール）

## 注意事項

- データ投入は外部キー制約を考慮して適切な順序で実行されます
- 既存データがある場合、重複を避けるためUPSERT処理が行われます
- `--reset` オプション使用時は既存データが削除されるため注意してください
