# Phase 5: データ移行・バッチ処理 (Week 11)

## フェーズ概要

- **期間**: Week 11 (1週間)
- **目標**: 既存データの移行とバッチ処理システムの実装
- **成果物**: データ移行機能、バッチ処理システム、実データ移行完了

## タスク一覧

### TASK-032: CSV取込み機能

- **説明**: 既存データのCSV取込み
- **優先度**: High
- **見積工数**: 12h
- **状態**: 未着手
- **依存関係**: TASK-011

#### 実装内容(TASK-032)

1. CSV パーサー実装
2. データ検証機能
3. エラーハンドリング
4. 進捗表示機能

#### CSV取込み実装

```rust
// src/services/csv_import_service.rs
use csv::ReaderBuilder;
use serde::Deserialize;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct DocumentCsvRecord {
    pub title: String,
    pub document_type_code: String,
    pub business_number: Option<String>,
    pub creator_name: String,
    pub created_date: String,
    pub internal_external: Option<String>,
    pub importance_class: Option<String>,
    pub personal_info: Option<String>,
    pub notes: Option<String>,
}

pub struct CsvImportService {
    document_service: Box<dyn DocumentService>,
    employee_service: Box<dyn EmployeeService>,
    document_type_service: Box<dyn DocumentTypeService>,
}

#[derive(Debug)]
pub struct ImportResult {
    pub total_records: usize,
    pub successful_imports: usize,
    pub failed_imports: usize,
    pub errors: Vec<ImportError>,
}

#[derive(Debug)]
pub struct ImportError {
    pub row_number: usize,
    pub field: Option<String>,
    pub message: String,
    pub raw_data: String,
}

impl CsvImportService {
    pub async fn import_documents_from_csv<R: Read>(
        &self,
        reader: R,
        options: ImportOptions,
    ) -> Result<ImportResult, CsvImportError> {
        let mut csv_reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(reader);
        
        let mut result = ImportResult {
            total_records: 0,
            successful_imports: 0,
            failed_imports: 0,
            errors: Vec::new(),
        };
        
        // ヘッダー検証
        let headers = csv_reader.headers()?;
        self.validate_headers(headers)?;
        
        for (row_index, record_result) in csv_reader.deserialize::<DocumentCsvRecord>().enumerate() {
            result.total_records += 1;
            
            match record_result {
                Ok(record) => {
                    match self.process_document_record(record, row_index + 2, &options).await {
                        Ok(_) => result.successful_imports += 1,
                        Err(error) => {
                            result.failed_imports += 1;
                            result.errors.push(error);
                        }
                    }
                }
                Err(csv_error) => {
                    result.failed_imports += 1;
                    result.errors.push(ImportError {
                        row_number: row_index + 2,
                        field: None,
                        message: format!("CSV parsing error: {}", csv_error),
                        raw_data: String::new(),
                    });
                }
            }
        }
        
        Ok(result)
    }
    
    async fn process_document_record(
        &self,
        record: DocumentCsvRecord,
        row_number: usize,
        options: &ImportOptions,
    ) -> Result<Document, ImportError> {
        // データ検証
        self.validate_record(&record, row_number)?;
        
        // 関連データの解決
        let document_type = self.resolve_document_type(&record.document_type_code)
            .await
            .map_err(|e| ImportError {
                row_number,
                field: Some("document_type_code".to_string()),
                message: format!("Document type not found: {}", e),
                raw_data: record.document_type_code.clone(),
            })?;
        
        let creator = self.resolve_creator(&record.creator_name)
            .await
            .map_err(|e| ImportError {
                row_number,
                field: Some("creator_name".to_string()),
                message: format!("Creator not found or ambiguous: {}", e),
                raw_data: record.creator_name.clone(),
            })?;
        
        let created_date = chrono::NaiveDate::parse_from_str(&record.created_date, "%Y-%m-%d")
            .map_err(|e| ImportError {
                row_number,
                field: Some("created_date".to_string()),
                message: format!("Invalid date format: {}", e),
                raw_data: record.created_date.clone(),
            })?;
        
        // 文書作成
        let create_request = CreateDocumentRequest {
            number: None, // 自動生成
            title: record.title,
            document_type_id: document_type.id,
            business_number: record.business_number,
            created_by: creator.id,
            created_date,
            internal_external: record.internal_external,
            importance_class: record.importance_class,
            personal_info: record.personal_info,
            notes: record.notes,
        };
        
        self.document_service.create_document(create_request, &options.user_permissions)
            .await
            .map_err(|e| ImportError {
                row_number,
                field: None,
                message: format!("Failed to create document: {}", e),
                raw_data: format!("{:?}", record),
            })
    }
    
    async fn resolve_creator(&self, creator_name: &str) -> Result<Employee, ResolveError> {
        let candidates = self.employee_service.search_by_name(creator_name).await?;
        
        match candidates.len() {
            0 => Err(ResolveError::NotFound),
            1 => Ok(candidates[0].clone()),
            _ => {
                // 複数候補がある場合は完全一致を探す
                for candidate in &candidates {
                    if candidate.name == creator_name {
                        return Ok(candidate.clone());
                    }
                }
                Err(ResolveError::Ambiguous(candidates))
            }
        }
    }
}
```

#### インポート進捗UI

```svelte
<!-- src/routes/admin/import/+page.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import ProgressBar from '$lib/components/ui/ProgressBar.svelte';
  import ErrorList from '$lib/components/ErrorList.svelte';
  
  let files: FileList;
  let isImporting = false;
  let importProgress = 0;
  let importResult: any = null;
  let importErrors: any[] = [];
  
  async function handleImport() {
    if (!files || files.length === 0) return;
    
    isImporting = true;
    importProgress = 0;
    importResult = null;
    importErrors = [];
    
    try {
      const formData = new FormData();
      formData.append('file', files[0]);
      
      const response = await fetch('/api/admin/import/documents', {
        method: 'POST',
        body: formData,
        headers: {
          'Authorization': `Bearer ${$authStore.token}`
        }
      });
      
      if (response.ok) {
        const result = await response.json();
        importResult = result;
        importErrors = result.errors || [];
      } else {
        throw new Error('Import failed');
      }
    } catch (error) {
      console.error('Import error:', error);
    } finally {
      isImporting = false;
    }
  }
</script>

<div class="container mx-auto px-4 py-8">
  <div class="max-w-4xl mx-auto">
    <h1 class="text-3xl font-bold text-gray-900 mb-8">データインポート</h1>
    
    <div class="bg-white shadow rounded-lg p-6">
      <h2 class="text-lg font-medium text-gray-900 mb-4">CSVファイルアップロード</h2>
      
      <div class="mb-6">
        <label for="csvFile" class="block text-sm font-medium text-gray-700 mb-2">
          CSVファイルを選択
        </label>
        <input
          id="csvFile"
          type="file"
          accept=".csv"
          bind:files
          class="block w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-full file:border-0 file:text-sm file:font-semibold file:bg-blue-50 file:text-blue-700 hover:file:bg-blue-100"
        />
      </div>
      
      {#if isImporting}
        <div class="mb-6">
          <div class="flex justify-between text-sm text-gray-600 mb-2">
            <span>インポート進行中...</span>
            <span>{importProgress}%</span>
          </div>
          <ProgressBar progress={importProgress} />
        </div>
      {/if}
      
      <Button
        on:click={handleImport}
        disabled={!files || files.length === 0 || isImporting}
        loading={isImporting}
      >
        インポート開始
      </Button>
    </div>
    
    {#if importResult}
      <div class="mt-8 bg-white shadow rounded-lg p-6">
        <h2 class="text-lg font-medium text-gray-900 mb-4">インポート結果</h2>
        
        <div class="grid grid-cols-3 gap-4 mb-6">
          <div class="text-center p-4 bg-gray-50 rounded">
            <div class="text-2xl font-bold text-gray-900">{importResult.total_records}</div>
            <div class="text-sm text-gray-600">総レコード数</div>
          </div>
          <div class="text-center p-4 bg-green-50 rounded">
            <div class="text-2xl font-bold text-green-600">{importResult.successful_imports}</div>
            <div class="text-sm text-gray-600">成功</div>
          </div>
          <div class="text-center p-4 bg-red-50 rounded">
            <div class="text-2xl font-bold text-red-600">{importResult.failed_imports}</div>
            <div class="text-sm text-gray-600">失敗</div>
          </div>
        </div>
        
        {#if importErrors.length > 0}
          <ErrorList errors={importErrors} />
        {/if}
      </div>
    {/if}
  </div>
</div>
```

#### 成果物(TASK-032)

- CSV取込み機能
- データ検証システム
- エラーハンドリング
- インポート進捗UI

---

### TASK-033: 名寄せ機能実装

- **説明**: 業務番号・作成者・顧客の名寄せ
- **優先度**: High
- **見積工数**: 20h
- **状態**: 未着手
- **依存関係**: TASK-032

#### 実装内容(TASK-033)

1. 文字列類似度計算
2. 名寄せルールエンジン
3. 手動確認機能
4. 統合処理

#### 名寄せ実装

```rust
// src/services/deduplication_service.rs
use strsim::jaro_winkler;

pub struct DeduplicationService {
    similarity_threshold: f64,
    repository: Box<dyn DeduplicationRepository>,
}

#[derive(Debug, Clone)]
pub struct DuplicationCandidate {
    pub original_id: i32,
    pub duplicate_id: i32,
    pub similarity_score: f64,
    pub field_name: String,
    pub original_value: String,
    pub duplicate_value: String,
}

impl DeduplicationService {
    pub async fn find_employee_duplicates(&self) -> Result<Vec<DuplicationCandidate>, DeduplicationError> {
        let employees = self.repository.get_all_employees().await?;
        let mut candidates = Vec::new();
        
        for i in 0..employees.len() {
            for j in (i + 1)..employees.len() {
                let similarity = jaro_winkler(&employees[i].name, &employees[j].name);
                
                if similarity >= self.similarity_threshold {
                    candidates.push(DuplicationCandidate {
                        original_id: employees[i].id,
                        duplicate_id: employees[j].id,
                        similarity_score: similarity,
                        field_name: "name".to_string(),
                        original_value: employees[i].name.clone(),
                        duplicate_value: employees[j].name.clone(),
                    });
                }
            }
        }
        
        Ok(candidates)
    }
    
    pub async fn find_customer_duplicates(&self) -> Result<Vec<DuplicationCandidate>, DeduplicationError> {
        let customers = self.repository.get_all_customers().await?;
        let mut candidates = Vec::new();
        
        for i in 0..customers.len() {
            for j in (i + 1)..customers.len() {
                // 会社名の類似度チェック
                let name_similarity = jaro_winkler(&customers[i].name, &customers[j].name);
                if name_similarity >= self.similarity_threshold {
                    candidates.push(DuplicationCandidate {
                        original_id: customers[i].id,
                        duplicate_id: customers[j].id,
                        similarity_score: name_similarity,
                        field_name: "name".to_string(),
                        original_value: customers[i].name.clone(),
                        duplicate_value: customers[j].name.clone(),
                    });
                }
                
                // 住所の類似度チェック
                if let (Some(addr1), Some(addr2)) = (&customers[i].address, &customers[j].address) {
                    let addr_similarity = jaro_winkler(addr1, addr2);
                    if addr_similarity >= self.similarity_threshold {
                        candidates.push(DuplicationCandidate {
                            original_id: customers[i].id,
                            duplicate_id: customers[j].id,
                            similarity_score: addr_similarity,
                            field_name: "address".to_string(),
                            original_value: addr1.clone(),
                            duplicate_value: addr2.clone(),
                        });
                    }
                }
            }
        }
        
        Ok(candidates)
    }
    
    pub async fn merge_employees(
        &self,
        primary_id: i32,
        duplicate_ids: Vec<i32>,
        user_id: i32,
    ) -> Result<MergeResult, DeduplicationError> {
        let mut transaction = self.repository.begin_transaction().await?;
        
        // 重複データの文書を主データに移管
        for duplicate_id in &duplicate_ids {
            self.repository.update_document_creator(
                &mut transaction,
                *duplicate_id,
                primary_id,
            ).await?;
        }
        
        // 重複データを論理削除
        for duplicate_id in &duplicate_ids {
            self.repository.soft_delete_employee(
                &mut transaction,
                *duplicate_id,
                user_id,
            ).await?;
        }
        
        // マージ履歴記録
        let merge_record = MergeRecord {
            primary_id,
            duplicate_ids: duplicate_ids.clone(),
            merged_by: user_id,
            merge_type: MergeType::Employee,
        };
        
        self.repository.record_merge(
            &mut transaction,
            merge_record,
        ).await?;
        
        transaction.commit().await?;
        
        Ok(MergeResult {
            primary_id,
            merged_count: duplicate_ids.len(),
            affected_documents: self.count_affected_documents(primary_id).await?,
        })
    }
}
```

#### 名寄せUI

```svelte
<!-- src/routes/admin/deduplication/+page.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import Modal from '$lib/components/ui/Modal.svelte';
  
  let duplicateCandidates: any[] = [];
  let selectedCandidates: Set<string> = new Set();
  let isLoading = false;
  let showMergeModal = false;
  let mergeTarget: any = null;
  
  async function findDuplicates(type: 'employees' | 'customers') {
    isLoading = true;
    try {
      const response = await fetch(`/api/admin/deduplication/${type}`);
      if (response.ok) {
        duplicateCandidates = await response.json();
      }
    } catch (error) {
      console.error('Failed to find duplicates:', error);
    } finally {
      isLoading = false;
    }
  }
  
  function handleMerge(candidate: any) {
    mergeTarget = candidate;
    showMergeModal = true;
  }
  
  async function confirmMerge() {
    if (!mergeTarget) return;
    
    try {
      const response = await fetch('/api/admin/deduplication/merge', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          primaryId: mergeTarget.original_id,
          duplicateIds: [mergeTarget.duplicate_id]
        })
      });
      
      if (response.ok) {
        // 候補リストから削除
        duplicateCandidates = duplicateCandidates.filter(
          c => c !== mergeTarget
        );
        showMergeModal = false;
        mergeTarget = null;
      }
    } catch (error) {
      console.error('Merge failed:', error);
    }
  }
  
  onMount(() => {
    findDuplicates('employees');
  });
</script>

<div class="container mx-auto px-4 py-8">
  <h1 class="text-3xl font-bold text-gray-900 mb-8">重複データ統合</h1>
  
  <div class="mb-6 space-x-4">
    <Button on:click={() => findDuplicates('employees')} {isLoading}>
      社員重複検索
    </Button>
    <Button on:click={() => findDuplicates('customers')} {isLoading}>
      顧客重複検索
    </Button>
  </div>
  
  {#if duplicateCandidates.length > 0}
    <div class="bg-white shadow rounded-lg overflow-hidden">
      <table class="min-w-full divide-y divide-gray-200">
        <thead class="bg-gray-50">
          <tr>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              フィールド
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              元データ
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              重複候補
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              類似度
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              操作
            </th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          {#each duplicateCandidates as candidate}
            <tr>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                {candidate.field_name}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                {candidate.original_value}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                {candidate.duplicate_value}
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <span class="inline-flex px-2 py-1 text-xs font-semibold rounded-full 
                           {candidate.similarity_score >= 0.9 ? 'bg-red-100 text-red-800' : 
                            candidate.similarity_score >= 0.8 ? 'bg-yellow-100 text-yellow-800' : 
                            'bg-green-100 text-green-800'}">
                  {(candidate.similarity_score * 100).toFixed(1)}%
                </span>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm space-x-2">
                <Button
                  size="sm"
                  on:click={() => handleMerge(candidate)}
                >
                  統合
                </Button>
                <Button
                  size="sm"
                  variant="secondary"
                  on:click={() => ignoreDuplicate(candidate)}
                >
                  無視
                </Button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {:else if !isLoading}
    <div class="text-center py-12">
      <p class="text-gray-500">重複候補が見つかりませんでした。</p>
    </div>
  {/if}
</div>

{#if showMergeModal}
  <Modal on:close={() => showMergeModal = false}>
    <div class="p-6">
      <h3 class="text-lg font-medium text-gray-900 mb-4">データ統合の確認</h3>
      
      <div class="mb-6">
        <p class="text-sm text-gray-600 mb-2">以下のデータを統合しますか？</p>
        <div class="bg-gray-50 p-4 rounded">
          <p><strong>元データ:</strong> {mergeTarget?.original_value}</p>
          <p><strong>統合先:</strong> {mergeTarget?.duplicate_value}</p>
          <p><strong>類似度:</strong> {(mergeTarget?.similarity_score * 100).toFixed(1)}%</p>
        </div>
      </div>
      
      <div class="flex justify-end space-x-4">
        <Button variant="secondary" on:click={() => showMergeModal = false}>
          キャンセル
        </Button>
        <Button on:click={confirmMerge}>
          統合実行
        </Button>
      </div>
    </div>
  </Modal>
{/if}
```

#### 成果物(TASK-033)

- 重複データ検出機能
- 名寄せルールエンジン
- 手動確認UI
- データ統合機能

---

### TASK-034: バッチ処理実装

- **説明**: 月次ファイル確認バッチ
- **優先度**: High
- **見積工数**: 16h
- **状態**: 未着手
- **依存関係**: TASK-028

#### 実装内容(TASK-034)

1. スケジューラー実装
2. バッチジョブ管理
3. 進捗監視機能
4. 結果レポート

#### バッチ処理実装

```rust
// src/batch/scheduler.rs
use tokio_cron_scheduler::{JobScheduler, Job};
use std::sync::Arc;

pub struct BatchScheduler {
    scheduler: JobScheduler,
    file_check_service: Arc<FileCheckService>,
    notification_service: Arc<NotificationService>,
}

impl BatchScheduler {
    pub async fn new(
        file_check_service: Arc<FileCheckService>,
        notification_service: Arc<NotificationService>,
    ) -> Result<Self, BatchError> {
        let scheduler = JobScheduler::new().await?;
        
        Ok(Self {
            scheduler,
            file_check_service,
            notification_service,
        })
    }
    
    pub async fn start(&self) -> Result<(), BatchError> {
        // 月次ファイル確認バッチ（毎月1日 9:00実行）
        let file_check_service = Arc::clone(&self.file_check_service);
        let notification_service = Arc::clone(&self.notification_service);
        
        let monthly_file_check = Job::new_async("0 0 9 1 * *", move |_uuid, _l| {
            let file_check_service = Arc::clone(&file_check_service);
            let notification_service = Arc::clone(&notification_service);
            
            Box::pin(async move {
                tracing::info!("Starting monthly file check batch");
                
                match run_monthly_file_check(file_check_service, notification_service).await {
                    Ok(result) => {
                        tracing::info!("Monthly file check completed: {:?}", result);
                    }
                    Err(error) => {
                        tracing::error!("Monthly file check failed: {:?}", error);
                    }
                }
            })
        })?;
        
        self.scheduler.add(monthly_file_check).await?;
        
        // 週次AD同期バッチ（毎週月曜日 6:00実行）
        let weekly_ad_sync = Job::new_async("0 0 6 * * MON", move |_uuid, _l| {
            Box::pin(async move {
                tracing::info!("Starting weekly AD sync batch");
                // AD同期処理
            })
        })?;
        
        self.scheduler.add(weekly_ad_sync).await?;
        
        self.scheduler.start().await?;
        
        Ok(())
    }
}

async fn run_monthly_file_check(
    file_check_service: Arc<FileCheckService>,
    notification_service: Arc<NotificationService>,
) -> Result<FileCheckBatchResult, BatchError> {
    let start_time = chrono::Utc::now();
    
    // 対象文書の取得
    let documents = file_check_service.get_check_target_documents().await?;
    let total_documents = documents.len();
    
    let mut results = FileCheckBatchResult {
        total_documents,
        checked_documents: 0,
        missing_folders: 0,
        missing_approvals: 0,
        errors: Vec::new(),
        start_time,
        end_time: start_time,
    };
    
    // バッチ実行記録作成
    let batch_execution = BatchExecution {
        batch_type: BatchType::FileCheck,
        start_time,
        status: BatchStatus::Running,
        total_items: total_documents,
        processed_items: 0,
    };
    
    let execution_id = file_check_service.create_batch_execution(batch_execution).await?;
    
    // 文書ごとにファイル確認実行
    for (index, document) in documents.iter().enumerate() {
        match file_check_service.check_document_existence(document).await {
            Ok(check_result) => {
                results.checked_documents += 1;
                
                if !check_result.folder_exists {
                    results.missing_folders += 1;
                }
                
                if let Some(false) = check_result.approval_exists {
                    results.missing_approvals += 1;
                }
                
                // 不存在ファイルの通知
                if !check_result.folder_exists || check_result.approval_exists == Some(false) {
                    if let Err(notification_error) = notification_service
                        .send_file_missing_notification(document, &check_result)
                        .await
                    {
                        tracing::warn!("Failed to send notification: {:?}", notification_error);
                    }
                }
            }
            Err(error) => {
                results.errors.push(FileCheckError {
                    document_id: document.id,
                    document_number: document.number.clone(),
                    error_message: error.to_string(),
                });
            }
        }
        
        // 進捗更新（100件毎）
        if (index + 1) % 100 == 0 {
            file_check_service.update_batch_progress(
                execution_id,
                index + 1,
            ).await?;
        }
    }
    
    results.end_time = chrono::Utc::now();
    
    // バッチ完了記録
    file_check_service.complete_batch_execution(
        execution_id,
        BatchStatus::Completed,
        Some(serde_json::to_string(&results)?),
    ).await?;
    
    // サマリー通知
    notification_service.send_file_check_summary(
        results.total_documents,
        results.missing_folders,
        results.missing_approvals,
    ).await?;
    
    Ok(results)
}
```

#### バッチ管理UI

```svelte
<!-- src/routes/admin/batch/+page.svelte -->
<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import ProgressBar from '$lib/components/ui/ProgressBar.svelte';
  
  let batchExecutions: any[] = [];
  let isLoading = false;
  let refreshInterval: number;
  
  async function loadBatchExecutions() {
    isLoading = true;
    try {
      const response = await fetch('/api/admin/batch/executions');
      if (response.ok) {
        batchExecutions = await response.json();
      }
    } catch (error) {
      console.error('Failed to load batch executions:', error);
    } finally {
      isLoading = false;
    }
  }
  
  async function runBatch(batchType: string) {
    try {
      const response = await fetch(`/api/admin/batch/run/${batchType}`, {
        method: 'POST'
      });
      
      if (response.ok) {
        await loadBatchExecutions();
      }
    } catch (error) {
      console.error('Failed to run batch:', error);
    }
  }
  
  function formatDuration(startTime: string, endTime?: string) {
    const start = new Date(startTime);
    const end = endTime ? new Date(endTime) : new Date();
    const duration = end.getTime() - start.getTime();
    
    const hours = Math.floor(duration / (1000 * 60 * 60));
    const minutes = Math.floor((duration % (1000 * 60 * 60)) / (1000 * 60));
    const seconds = Math.floor((duration % (1000 * 60)) / 1000);
    
    return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
  }
  
  onMount(() => {
    loadBatchExecutions();
    // 5秒毎に自動更新
    refreshInterval = setInterval(loadBatchExecutions, 5000);
  });
  
  onDestroy(() => {
    if (refreshInterval) {
      clearInterval(refreshInterval);
    }
  });
</script>

<div class="container mx-auto px-4 py-8">
  <div class="flex justify-between items-center mb-8">
    <h1 class="text-3xl font-bold text-gray-900">バッチ処理管理</h1>
    <Button on:click={loadBatchExecutions} {isLoading}>
      更新
    </Button>
  </div>
  
  <!-- 手動実行ボタン -->
  <div class="mb-8 bg-white shadow rounded-lg p-6">
    <h2 class="text-lg font-medium text-gray-900 mb-4">手動実行</h2>
    <div class="space-x-4">
      <Button on:click={() => runBatch('file-check')}>
        ファイル存在確認
      </Button>
      <Button on:click={() => runBatch('ad-sync')}>
        AD同期
      </Button>
      <Button on:click={() => runBatch('data-cleanup')}>
        データクリーンアップ
      </Button>
    </div>
  </div>
  
  <!-- 実行履歴 -->
  <div class="bg-white shadow rounded-lg overflow-hidden">
    <div class="px-6 py-4 border-b border-gray-200">
      <h2 class="text-lg font-medium text-gray-900">実行履歴</h2>
    </div>
    
    <div class="overflow-x-auto">
      <table class="min-w-full divide-y divide-gray-200">
        <thead class="bg-gray-50">
          <tr>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              バッチ種別
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              ステータス
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              進捗
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              開始時刻
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              実行時間
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              結果
            </th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          {#each batchExecutions as execution}
            <tr>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                {execution.batch_type}
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <span class="inline-flex px-2 py-1 text-xs font-semibold rounded-full 
                           {execution.status === 'completed' ? 'bg-green-100 text-green-800' : 
                            execution.status === 'running' ? 'bg-yellow-100 text-yellow-800' : 
                            execution.status === 'failed' ? 'bg-red-100 text-red-800' : 
                            'bg-gray-100 text-gray-800'}">
                  {execution.status}
                </span>
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                {#if execution.status === 'running'}
                  <div class="w-32">
                    <ProgressBar 
                      progress={(execution.processed_items / execution.total_items) * 100}
                      showText={false}
                    />
                    <div class="text-xs text-gray-500 mt-1">
                      {execution.processed_items} / {execution.total_items}
                    </div>
                  </div>
                {:else}
                  <span class="text-sm text-gray-900">
                    {execution.processed_items} / {execution.total_items}
                  </span>
                {/if}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                {new Date(execution.start_time).toLocaleString()}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                {formatDuration(execution.start_time, execution.end_time)}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm">
                {#if execution.result}
                  <Button
                    size="sm"
                    variant="secondary"
                    on:click={() => showResult(execution)}
                  >
                    詳細
                  </Button>
                {:else}
                  -
                {/if}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>
</div>
```

#### 成果物(TASK-034)

- バッチスケジューラー
- ジョブ管理システム
- 進捗監視機能
- 実行履歴管理

---

### TASK-035: データ検証機能

- **説明**: 整合性チェック・レポート
- **優先度**: Medium
- **見積工数**: 8h
- **状態**: 未着手
- **依存関係**: TASK-033

#### 成果物(TASK-035)

- データ整合性チェック
- 検証レポート
- 自動修復機能

---

### TASK-036: バックアップ機能

- **説明**: 自動バックアップ・リストア
- **優先度**: Medium
- **見積工数**: 8h
- **状態**: 未着手
- **依存関係**: TASK-034

#### 成果物(TASK-036)

- 自動バックアップ機能
- リストア機能
- バックアップ監視

---

### TASK-037: 移行計画実行

- **説明**: 実データ移行・検証
- **優先度**: High
- **見積工数**: 12h
- **状態**: 未着手
- **依存関係**: TASK-035

#### 成果物(TASK-037)

- 実データ移行完了
- 移行検証レポート
- 本番データ準備完了

## フェーズ完了基準

### 必須条件

- [ ] CSV取込み機能が動作する
- [ ] 名寄せ機能が正常に動作する
- [ ] 月次バッチが自動実行される
- [ ] データ整合性が確保される
- [ ] 実データ移行が完了する

### 検証方法

```bash
# CSV取込みテスト
curl -X POST /api/admin/import/documents -F "file=@test.csv"

# バッチ実行テスト
curl -X POST /api/admin/batch/run/file-check

# データ検証
curl /api/admin/data/validation-report
```

## 次フェーズへの引き継ぎ事項

- 完全なデータ移行完了
- バッチ処理システム稼働
- テスト・品質保証準備
- 本番運用準備

## リスク・課題

- **データ品質**: 既存データの品質問題
- **移行時間**: 大量データの移行時間
- **バッチ性能**: 月次処理の実行時間

## 対応策

- 段階的データクリーニング
- 並列処理による高速化
- バッチ処理の最適化
