use crate::error::DeduplicationError;
use crate::models::Employee;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use strsim::jaro_winkler;
use tracing::info;
use uuid::Uuid;

/// 重複候補
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicationCandidate {
    pub id: Uuid,
    pub candidate_type: DuplicationType,
    pub primary_id: i32,
    pub duplicate_id: i32,
    pub similarity_score: f64,
    pub field_name: String,
    pub primary_value: String,
    pub duplicate_value: String,
    pub status: DuplicationStatus,
    pub created_at: DateTime<Utc>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub reviewed_by: Option<i32>,
}

/// 重複タイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DuplicationType {
    Employee,
    Customer,
    BusinessNumber,
    Document,
}

/// 重複ステータス
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DuplicationStatus {
    Pending,   // 未確認
    Confirmed, // 重複確認済み
    Ignored,   // 無視
    Merged,    // 統合済み
}

/// 統合結果
#[derive(Debug, Serialize)]
pub struct MergeResult {
    pub merge_id: Uuid,
    pub primary_id: i32,
    pub merged_ids: Vec<i32>,
    pub merge_type: DuplicationType,
    pub affected_documents: i32,
    pub merged_at: DateTime<Utc>,
    pub merged_by: i32,
}

/// 統合記録
#[derive(Debug, Serialize)]
pub struct MergeRecord {
    pub id: Uuid,
    pub primary_id: i32,
    pub duplicate_ids: Vec<i32>,
    pub merge_type: DuplicationType,
    pub merged_by: i32,
    pub merged_at: DateTime<Utc>,
    pub details: Option<String>,
}

/// 名寄せサービストレイト
#[async_trait]
pub trait DeduplicationService: Send + Sync {
    /// 社員の重複候補を検索
    async fn find_employee_duplicates(
        &self,
        threshold: f64,
    ) -> Result<Vec<DuplicationCandidate>, DeduplicationError>;

    /// 顧客の重複候補を検索
    async fn find_customer_duplicates(
        &self,
        threshold: f64,
    ) -> Result<Vec<DuplicationCandidate>, DeduplicationError>;

    /// 業務番号の重複候補を検索
    async fn find_business_number_duplicates(
        &self,
    ) -> Result<Vec<DuplicationCandidate>, DeduplicationError>;

    /// 文書の重複候補を検索
    async fn find_document_duplicates(
        &self,
        threshold: f64,
    ) -> Result<Vec<DuplicationCandidate>, DeduplicationError>;

    /// 重複候補のステータス更新
    async fn update_duplication_status(
        &self,
        candidate_id: Uuid,
        status: DuplicationStatus,
        reviewed_by: i32,
    ) -> Result<(), DeduplicationError>;

    /// 社員データの統合
    async fn merge_employees(
        &self,
        primary_id: i32,
        duplicate_ids: Vec<i32>,
        merged_by: i32,
    ) -> Result<MergeResult, DeduplicationError>;

    /// 統合履歴の取得
    async fn get_merge_history(
        &self,
        limit: Option<i32>,
    ) -> Result<Vec<MergeRecord>, DeduplicationError>;
}

/// 名寄せサービス実装
pub struct DeduplicationServiceImpl {
    // TODO: 実際のリポジトリを注入
}

impl DeduplicationServiceImpl {
    pub fn new() -> Self {
        Self {}
    }

    /// 文字列の類似度を計算
    fn calculate_similarity(&self, str1: &str, str2: &str) -> f64 {
        if str1.is_empty() && str2.is_empty() {
            return 1.0;
        }
        if str1.is_empty() || str2.is_empty() {
            return 0.0;
        }

        // 正規化（空白削除、小文字変換）
        let normalized1 = str1.trim().to_lowercase();
        let normalized2 = str2.trim().to_lowercase();

        // 完全一致チェック
        if normalized1 == normalized2 {
            return 1.0;
        }

        // Jaro-Winkler距離を使用
        jaro_winkler(&normalized1, &normalized2)
    }

    /// 部分一致による類似度計算
    fn calculate_partial_similarity(&self, str1: &str, str2: &str) -> f64 {
        let base_similarity = self.calculate_similarity(str1, str2);

        // 部分一致ボーナス
        let normalized1 = str1.trim().to_lowercase();
        let normalized2 = str2.trim().to_lowercase();

        let contains_bonus =
            if normalized1.contains(&normalized2) || normalized2.contains(&normalized1) {
                0.1
            } else {
                0.0
            };

        (base_similarity + contains_bonus).min(1.0)
    }

    /// カタカナ・ひらがなの類似度計算（日本語対応）
    fn calculate_japanese_similarity(&self, str1: &str, str2: &str) -> f64 {
        // カタカナをひらがなに変換して比較
        let hiragana1 = self.katakana_to_hiragana(str1);
        let hiragana2 = self.katakana_to_hiragana(str2);

        self.calculate_similarity(&hiragana1, &hiragana2)
    }

    /// カタカナをひらがなに変換（簡易実装）
    fn katakana_to_hiragana(&self, text: &str) -> String {
        text.chars()
            .map(|c| {
                if c >= 'ァ' && c <= 'ヶ' {
                    std::char::from_u32(c as u32 - 0x60).unwrap_or(c)
                } else {
                    c
                }
            })
            .collect()
    }

    /// 複合類似度計算（複数フィールドの重み付き平均）
    fn calculate_composite_similarity(&self, fields: &[(String, String, f64)]) -> f64 {
        if fields.is_empty() {
            return 0.0;
        }

        let total_weight: f64 = fields.iter().map(|(_, _, weight)| weight).sum();
        if total_weight == 0.0 {
            return 0.0;
        }

        let weighted_sum: f64 = fields
            .iter()
            .map(|(str1, str2, weight)| {
                let similarity = self.calculate_japanese_similarity(str1, str2);
                similarity * weight
            })
            .sum();

        weighted_sum / total_weight
    }

    /// 仮の社員データ取得
    async fn get_all_employees(&self) -> Result<Vec<Employee>, DeduplicationError> {
        // TODO: 実際のデータベースから取得
        Ok(vec![
            Employee {
                id: 1,
                employee_number: Some("001".to_string()),
                name: "山田太郎".to_string(),
                email: Some("yamada@company.com".to_string()),
                ad_username: Some("yamada.taro".to_string()),
                department_id: Some(1),
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            Employee {
                id: 2,
                employee_number: Some("002".to_string()),
                name: "山田 太郎".to_string(), // スペース違い
                email: Some("yamada.taro@company.com".to_string()),
                ad_username: Some("yamada_taro".to_string()),
                department_id: Some(1),
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            Employee {
                id: 3,
                employee_number: Some("003".to_string()),
                name: "佐藤花子".to_string(),
                email: Some("sato@company.com".to_string()),
                ad_username: Some("sato.hanako".to_string()),
                department_id: Some(2),
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            Employee {
                id: 4,
                employee_number: Some("004".to_string()),
                name: "サトウハナコ".to_string(), // カタカナ
                email: Some("hanako.sato@company.com".to_string()),
                ad_username: Some("h.sato".to_string()),
                department_id: Some(2),
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
        ])
    }
}

#[async_trait]
impl DeduplicationService for DeduplicationServiceImpl {
    async fn find_employee_duplicates(
        &self,
        threshold: f64,
    ) -> Result<Vec<DuplicationCandidate>, DeduplicationError> {
        info!(
            "Starting employee duplicate detection with threshold: {}",
            threshold
        );

        if threshold < 0.0 || threshold > 1.0 {
            return Err(DeduplicationError::InvalidThreshold { threshold });
        }

        let employees = self.get_all_employees().await?;
        let mut candidates = Vec::new();

        for i in 0..employees.len() {
            for j in (i + 1)..employees.len() {
                let emp1 = &employees[i];
                let emp2 = &employees[j];

                // 名前の類似度計算
                let name_similarity = self.calculate_japanese_similarity(&emp1.name, &emp2.name);

                if name_similarity >= threshold {
                    candidates.push(DuplicationCandidate {
                        id: Uuid::new_v4(),
                        candidate_type: DuplicationType::Employee,
                        primary_id: emp1.id,
                        duplicate_id: emp2.id,
                        similarity_score: name_similarity,
                        field_name: "name".to_string(),
                        primary_value: emp1.name.clone(),
                        duplicate_value: emp2.name.clone(),
                        status: DuplicationStatus::Pending,
                        created_at: Utc::now(),
                        reviewed_at: None,
                        reviewed_by: None,
                    });
                }

                // メールアドレスの類似度計算
                if let (Some(email1), Some(email2)) = (&emp1.email, &emp2.email) {
                    let email_similarity = self.calculate_similarity(email1, email2);
                    if email_similarity >= threshold {
                        candidates.push(DuplicationCandidate {
                            id: Uuid::new_v4(),
                            candidate_type: DuplicationType::Employee,
                            primary_id: emp1.id,
                            duplicate_id: emp2.id,
                            similarity_score: email_similarity,
                            field_name: "email".to_string(),
                            primary_value: email1.clone(),
                            duplicate_value: email2.clone(),
                            status: DuplicationStatus::Pending,
                            created_at: Utc::now(),
                            reviewed_at: None,
                            reviewed_by: None,
                        });
                    }
                }

                // 複合類似度計算（名前とメールの重み付き平均）
                if let (Some(email1), Some(email2)) = (&emp1.email, &emp2.email) {
                    let composite_similarity = self.calculate_composite_similarity(&[
                        (emp1.name.clone(), emp2.name.clone(), 0.7), // 名前の重み70%
                        (email1.clone(), email2.clone(), 0.3),       // メールの重み30%
                    ]);

                    if composite_similarity >= threshold {
                        candidates.push(DuplicationCandidate {
                            id: Uuid::new_v4(),
                            candidate_type: DuplicationType::Employee,
                            primary_id: emp1.id,
                            duplicate_id: emp2.id,
                            similarity_score: composite_similarity,
                            field_name: "composite".to_string(),
                            primary_value: format!("{} / {}", emp1.name, email1),
                            duplicate_value: format!("{} / {}", emp2.name, email2),
                            status: DuplicationStatus::Pending,
                            created_at: Utc::now(),
                            reviewed_at: None,
                            reviewed_by: None,
                        });
                    }
                }
            }
        }

        info!("Found {} employee duplicate candidates", candidates.len());
        Ok(candidates)
    }

    async fn find_customer_duplicates(
        &self,
        _threshold: f64,
    ) -> Result<Vec<DuplicationCandidate>, DeduplicationError> {
        // TODO: 顧客重複検索の実装
        info!("Customer duplicate detection not yet implemented");
        Ok(vec![])
    }

    async fn find_business_number_duplicates(
        &self,
    ) -> Result<Vec<DuplicationCandidate>, DeduplicationError> {
        // TODO: 業務番号重複検索の実装
        info!("Business number duplicate detection not yet implemented");
        Ok(vec![])
    }

    async fn find_document_duplicates(
        &self,
        _threshold: f64,
    ) -> Result<Vec<DuplicationCandidate>, DeduplicationError> {
        // TODO: 文書重複検索の実装
        info!("Document duplicate detection not yet implemented");
        Ok(vec![])
    }

    async fn update_duplication_status(
        &self,
        candidate_id: Uuid,
        status: DuplicationStatus,
        reviewed_by: i32,
    ) -> Result<(), DeduplicationError> {
        // TODO: データベース更新の実装
        info!(
            "Updating duplication status for {}: {:?} by user {}",
            candidate_id, status, reviewed_by
        );
        Ok(())
    }

    async fn merge_employees(
        &self,
        primary_id: i32,
        duplicate_ids: Vec<i32>,
        merged_by: i32,
    ) -> Result<MergeResult, DeduplicationError> {
        info!(
            "Merging employees: primary={}, duplicates={:?}, by user={}",
            primary_id, duplicate_ids, merged_by
        );

        // TODO: 実際の統合処理
        // 1. 重複データの文書を主データに移管
        // 2. 重複データを論理削除
        // 3. 統合履歴記録

        let merge_result = MergeResult {
            merge_id: Uuid::new_v4(),
            primary_id,
            merged_ids: duplicate_ids.clone(),
            merge_type: DuplicationType::Employee,
            affected_documents: 0, // TODO: 実際の影響文書数を計算
            merged_at: Utc::now(),
            merged_by,
        };

        info!("Employee merge completed: {:?}", merge_result.merge_id);
        Ok(merge_result)
    }

    async fn get_merge_history(
        &self,
        limit: Option<i32>,
    ) -> Result<Vec<MergeRecord>, DeduplicationError> {
        // TODO: 統合履歴の取得
        info!("Getting merge history with limit: {:?}", limit);
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_calculate_similarity() {
        let service = DeduplicationServiceImpl::new();

        // 完全一致
        assert_eq!(service.calculate_similarity("山田太郎", "山田太郎"), 1.0);

        // 空文字列
        assert_eq!(service.calculate_similarity("", ""), 1.0);
        assert_eq!(service.calculate_similarity("", "山田太郎"), 0.0);

        // 部分的な類似
        let similarity = service.calculate_similarity("山田太郎", "山田 太郎");
        assert!(similarity > 0.8); // スペースの違いは高い類似度

        // 全く違う文字列
        let similarity = service.calculate_similarity("山田太郎", "佐藤花子");
        assert!(similarity < 0.5);
    }

    #[tokio::test]
    async fn test_japanese_similarity() {
        let service = DeduplicationServiceImpl::new();

        // カタカナ・ひらがなの類似度
        let similarity = service.calculate_japanese_similarity("サトウハナコ", "さとうはなこ");
        assert!(similarity > 0.9);

        let similarity = service.calculate_japanese_similarity("佐藤花子", "サトウハナコ");
        assert!(similarity < 0.5); // 漢字とカタカナは低い類似度
    }

    #[tokio::test]
    async fn test_find_employee_duplicates() {
        let service = DeduplicationServiceImpl::new();

        let candidates = service.find_employee_duplicates(0.8).await.unwrap();
        assert!(!candidates.is_empty());

        // 類似度が閾値以上であることを確認
        for candidate in &candidates {
            assert!(candidate.similarity_score >= 0.8);
        }
    }

    #[test]
    fn test_invalid_threshold() {
        let service = DeduplicationServiceImpl::new();

        // 無効な閾値のテスト
        tokio_test::block_on(async {
            let result = service.find_employee_duplicates(-0.1).await;
            assert!(matches!(
                result,
                Err(DeduplicationError::InvalidThreshold { .. })
            ));

            let result = service.find_employee_duplicates(1.1).await;
            assert!(matches!(
                result,
                Err(DeduplicationError::InvalidThreshold { .. })
            ));
        });
    }
}
