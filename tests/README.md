# テスト構造

このディレクトリは以下の方法で整理されています：

## ディレクトリ構造

```
tests/
├── api/                    # API関連テスト
│   ├── api_handlers_test.rs    # APIハンドラーテスト
│   ├── graphql_api_test.rs     # GraphQL APIテスト
│   ├── graphql_types_test.rs   # GraphQLタイプ変換テスト
│   ├── http_api_test.rs        # HTTP APIテスト
│   └── mod.rs
├── batch/                  # バッチ処理テスト
│   ├── batch_processing_extended_test.rs
│   ├── batch_scheduler_test.rs
│   ├── batch_simple_test.rs
│   └── mod.rs
├── handlers/               # ハンドラー層テスト
│   ├── backup_handler_test.rs
│   ├── migration_handler_test.rs
│   └── mod.rs
├── integration/            # 統合テスト
│   ├── backup_test.rs          # バックアップ統合テスト
│   ├── database_integration_test.rs  # データベース統合テスト
│   ├── migration_test.rs       # マイグレーション統合テスト
│   └── mod.rs
├── models/                 # モデル層テスト
│   ├── backup_models_extended_test.rs
│   ├── document_clean_test.rs
│   ├── document_model_test.rs
│   ├── document_test.rs
│   ├── document_type_test.rs
│   └── mod.rs
├── services/               # サービス層テスト
│   ├── document_number_generator_service_test.rs
│   ├── migration_service_test.rs
│   ├── report_service_fixed_test.rs
│   ├── validation_service_extended_test.rs
│   ├── validation_test.rs
│   └── mod.rs
├── unit/                   # ユニットテスト
│   ├── basic_error_test.rs
│   ├── document_number_basic_test.rs
│   ├── document_number_generation_test.rs
│   ├── document_repository_test.rs
│   └── mod.rs
├── api_tests.rs           # APIテストエントリーポイント
├── batch_tests.rs         # バッチテストエントリーポイント
├── handlers_tests.rs      # ハンドラーテストエントリーポイント
├── integration_tests.rs   # 統合テストエントリーポイント
├── models_tests.rs        # モデルテストエントリーポイント
├── services_tests.rs      # サービステストエントリーポイント
├── unit_tests.rs          # ユニットテストエントリーポイント
└── README.md              # このファイル
```

## テスト実行方法

### 全テスト実行
```bash
cargo test
```

### カテゴリ別テスト実行
```bash
# API関連テスト
cargo test --test api_tests

# バッチ処理テスト
cargo test --test batch_tests

# ハンドラー層テスト
cargo test --test handlers_tests

# 統合テスト
cargo test --test integration_tests

# モデル層テスト
cargo test --test models_tests

# サービス層テスト
cargo test --test services_tests

# ユニットテスト
cargo test --test unit_tests
```

### 特定のテストファイル実行
```bash
# 例：文書関連のAPIテストのみ
cargo test --test api_tests api_handlers
```

## テスト分類基準

- **unit/**: 単一の関数やメソッドをテストする最小単位のテスト
- **models/**: データモデルの動作をテストするテスト
- **services/**: ビジネスロジックをテストするサービス層のテスト
- **handlers/**: HTTPリクエスト処理をテストするハンドラー層のテスト
- **api/**: API全体の動作をテストするテスト
- **batch/**: バッチ処理機能をテストするテスト
- **integration/**: システム全体の統合動作をテストするテスト