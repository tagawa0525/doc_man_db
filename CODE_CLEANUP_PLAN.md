# コードクリーンアップ計画

## 概要

実装コードにテスト用のコードが混在している問題を修正し、適切にコードを分離する。

## 現在の問題点

### 1. プロダクションコードでのテスト用エラーハンドリング

- `src/app.rs:31` - `.expect("Failed to connect to database")`
- `src/app.rs:38` - `.expect("Failed to create department repository")`

**問題**: `.expect()` はテスト用途に適しているが、本番環境では適切なエラーハンドリングが必要。

### 2. 実装ファイルに混在するテストモジュール

以下のファイルで `#[cfg(test)]` モジュールが実装コードと同じファイル内に存在：

- `src/handlers/advanced_search.rs` (183-200行)
- `src/services/validation_service.rs` (321-384行)  
- `src/config/mod.rs` (309-376行)
- `src/handlers/business_search.rs` (259-279行)
- `src/handlers/deduplication.rs` (375-378行)
- `src/handlers/batch.rs` (274-304行)
- `src/services/notification_service.rs` (323-385行)
- `src/services/document_number_generator.rs` (141-195行)

## 修正方針

### 1. エラーハンドリングの改善

- `src/app.rs` の `.expect()` を適切な `Result` 型のエラーハンドリングに変更
- アプリケーション初期化時のエラーを適切にログ出力し、グレースフルな終了処理を実装

### 2. テストコードの分離

- 各実装ファイルの `#[cfg(test)]` モジュールを `tests/unit/` ディレクトリの対応するファイルに移動
- テストコードは以下の構造で整理：

  ```tree
  tests/
  ├── unit/
  │   ├── handlers/
  │   │   ├── advanced_search_test.rs
  │   │   ├── business_search_test.rs
  │   │   ├── batch_test.rs
  │   │   └── deduplication_test.rs
  │   ├── services/
  │   │   ├── validation_service_test.rs
  │   │   ├── notification_service_test.rs
  │   │   └── document_number_generator_test.rs
  │   └── config/
  │       └── config_test.rs
  ```

### 3. テスト実行の確認

- 分離後、すべてのテストが正常に実行されることを確認
- `cargo test` でテストスイートが完全に動作することを検証

## 実装順序

1. **方針ドキュメント作成** ✓
2. **エラーハンドリング修正** - `src/app.rs`
3. **テストコード分離** - 各モジュールから `tests/unit/` へ移動
4. **テスト実行確認** - `cargo test` で動作検証

## 期待される効果

- **コードの明確な分離**: 実装コードとテストコードが明確に分かれる
- **メンテナンス性向上**: テストコードが独立しており、変更時の影響範囲が明確
- **本番環境での品質向上**: 適切なエラーハンドリングによる安定性向上
- **開発効率改善**: テストコードが整理され、新規テスト追加が容易
