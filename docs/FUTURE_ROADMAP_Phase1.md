# 将来展望・拡張計画

## 概要

本文書は、文書管理システムの初期リリース後の将来的な機能拡張・改善計画をまとめたものです。現在の280時間開発計画完了後、システムの成熟度とユーザーニーズに応じて段階的に実装を検討します。

---

## Phase 9+: 将来拡張計画

### 🔧 技術的改善・リファクタリング (優先度: High)

#### 日付・時刻処理の統一化

- **課題**: 文字列ベースの日付解析による性能低下とコード重複
- **解決策**: SQLxの native chrono サポート活用
- **実装工数**: 4-6時間
- **実装時期**: リリース後1ヶ月以内（技術的負債）

```rust
// 現状の問題: 手動文字列解析
created_date: NaiveDate::parse_from_str(&row.get::<String, _>("created_date"), "%Y-%m-%d")

// 改善案: SQLx の自動変換活用
#[derive(FromRow)]
struct Document {
    created_date: NaiveDate,  // SQLx が自動変換
    created_at: NaiveDateTime,
}
```

#### サービス層の導入

- **課題**: Repository層での直接バリデーション、関心の分離不足
- **解決策**: ビジネスロジック専用のService層実装
- **実装工数**: 8-12時間
- **実装時期**: リリース後2-3ヶ月

```rust
// 実装イメージ
pub struct DocumentService {
    repository: Arc<dyn DocumentRepository>,
    number_generator: Arc<dyn DocumentNumberGenerator>,
    validator: DocumentValidator,
}
```

#### エラーハンドリング改善

- **課題**: データベースエラーの内部実装詳細露出
- **解決策**: アプリケーション固有エラー型の導入
- **実装工数**: 6-8時間
- **実装時期**: リリース後1-2ヶ月

```rust
// 改善案
#[derive(Debug, thiserror::Error)]
pub enum BusinessLogicError {
    #[error("Document not found")]
    DocumentNotFound,
    #[error("Access denied")]
    AccessDenied,
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
}
```

#### クエリビルダー導入

- **課題**: 動的SQL文字列連結による保守性・安全性の問題
- **解決策**: 型安全なクエリビルダーパターン実装
- **実装工数**: 10-15時間
- **実装時期**: リリース後3-4ヶ月

```rust
// 実装イメージ
pub struct DocumentQueryBuilder {
    filters: Vec<Filter>,
    sorts: Vec<Sort>,
    pagination: Option<Pagination>,
}
```

#### 日付解析ヘルパー関数の抽出

- **課題**: 同一の日付解析ロジックが複数箇所で重複
- **解決策**: 共通ヘルパー関数への集約
- **実装工数**: 2-3時間
- **実装時期**: リリース後1ヶ月以内

```rust
// 実装イメージ
pub mod date_utils {
    pub fn parse_document_row(row: SqliteRow) -> Result<Document, RepositoryError> {
        // 中央化された解析ロジック
    }
}
```

### 🧪 テスト・品質保証強化 (優先度: Medium)

#### エラーシナリオテストの拡充

- **課題**: 正常系テスト中心、異常系テストが不足
- **解決策**: 包括的エラーハンドリングテストの実装
- **実装工数**: 8-12時間
- **実装時期**: リリース後2-3ヶ月

#### パフォーマンステストの導入

- **課題**: 性能回帰の早期検出手段が不足
- **解決策**: ベンチマークテストとCI/CD統合
- **実装工数**: 12-16時間
- **実装時期**: リリース後3-4ヶ月

#### Property-based テストの拡張

- **課題**: エッジケースの網羅的テストが不足
- **解決策**: プロパティテストによるロバストネス向上
- **実装工数**: 6-10時間
- **実装時期**: リリース後4-6ヶ月

### 🔍 コード品質・保守性向上 (優先度: Medium)

#### 設定値の外部化

- **課題**: ハードコーディングされた値（日付フォーマット等）
- **解決策**: 設定ファイル・環境変数による外部化
- **実装工数**: 4-6時間
- **実装時期**: リリース後2-3ヶ月

#### ドキュメント自動生成

- **課題**: APIドキュメントの手動メンテナンス負荷
- **解決策**: OpenAPI/GraphQL スキーマからの自動生成
- **実装工数**: 8-12時間
- **実装時期**: リリース後4-6ヶ月

#### 循環的複雑度削減

- **課題**: 一部メソッドの複雑度が高い（特に search メソッド）
- **解決策**: 小さなメソッドへの分割とリファクタリング
- **実装工数**: 6-8時間
- **実装時期**: リリース後2-4ヶ月
