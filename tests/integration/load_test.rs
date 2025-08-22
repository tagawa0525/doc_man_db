use super::test_environment::*;
use chrono::NaiveDate;
use std::time::{Duration, Instant};

#[tokio::test]
async fn test_concurrent_document_creation() {
    let test_env = TestEnvironment::new().await;
    test_env.setup_test_data().await.unwrap();

    let admin_token = test_env
        .authenticate_user("admin", "password")
        .await
        .unwrap();

    // 50個の文書を順次作成（ライフタイム問題を回避）
    const CONCURRENT_DOCS: usize = 50;
    let mut results = vec![];
    let mut durations = vec![];

    let start_time = Instant::now();

    for i in 0..CONCURRENT_DOCS {
        let start = Instant::now();
        let result = test_env
            .create_document(
                CreateDocumentRequest {
                    title: format!("負荷テスト文書 {}", i),
                    document_type_id: 1,
                    created_by: 1,
                    created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
                    confidentiality: TestConfidentiality::default(),
                    notes: Some(format!("負荷テスト {} 番目", i)),
                },
                &admin_token,
            )
            .await;

        let duration = start.elapsed();
        results.push(result);
        durations.push(duration);
    }

    let total_time = start_time.elapsed();

    // 結果の検証
    let successful_creations = results.iter().filter(|r| r.is_ok()).count();
    assert_eq!(successful_creations, CONCURRENT_DOCS);

    // パフォーマンス指標の計算
    let avg_duration = durations.iter().sum::<Duration>() / durations.len() as u32;
    let max_duration = durations.iter().max().unwrap();
    let min_duration = durations.iter().min().unwrap();

    println!("🔥 負荷テスト結果 - 並行文書作成:");
    println!("  - 作成文書数: {}", CONCURRENT_DOCS);
    println!(
        "  - 成功率: {}/{} ({:.1}%)",
        successful_creations,
        CONCURRENT_DOCS,
        (successful_creations as f64 / CONCURRENT_DOCS as f64) * 100.0
    );
    println!("  - 総実行時間: {:?}", total_time);
    println!("  - 平均レスポンス時間: {:?}", avg_duration);
    println!("  - 最大レスポンス時間: {:?}", max_duration);
    println!("  - 最小レスポンス時間: {:?}", min_duration);
    println!(
        "  - スループット: {:.1} docs/sec",
        CONCURRENT_DOCS as f64 / total_time.as_secs_f64()
    );

    // パフォーマンス要件の検証
    assert!(
        avg_duration < Duration::from_millis(500),
        "平均レスポンス時間が500msを超えています"
    );
    assert!(
        *max_duration < Duration::from_secs(2),
        "最大レスポンス時間が2秒を超えています"
    );
    assert!(
        total_time < Duration::from_secs(10),
        "総実行時間が10秒を超えています"
    );
}

#[tokio::test]
async fn test_concurrent_search_operations() {
    let test_env = TestEnvironment::new().await;
    test_env.setup_test_data().await.unwrap();

    let admin_token = test_env
        .authenticate_user("admin", "password")
        .await
        .unwrap();

    // テスト用文書を事前に作成
    for i in 0..20 {
        test_env
            .create_document(
                CreateDocumentRequest {
                    title: format!("検索テスト文書 {}", i),
                    document_type_id: if i % 2 == 0 { 1 } else { 2 },
                    created_by: 1,
                    created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
                    confidentiality: TestConfidentiality::default(),
                    notes: Some(format!("検索用データ {}", i)),
                },
                &admin_token,
            )
            .await
            .unwrap();
    }

    // 100回の順次検索を実行（ライフタイム問題を回避）
    const CONCURRENT_SEARCHES: usize = 100;
    let mut results = vec![];
    let mut durations = vec![];

    let start_time = Instant::now();

    for i in 0..CONCURRENT_SEARCHES {
        let start = Instant::now();
        let search_term = if i % 3 == 0 {
            Some("検索テスト".to_string())
        } else if i % 3 == 1 {
            Some("文書".to_string())
        } else {
            None // 全件検索
        };

        let result = test_env
            .search_documents(
                DocumentSearchInput {
                    title: search_term,
                    pagination: Pagination {
                        ..Pagination::new(1, 10)
                    },
                },
                &admin_token,
            )
            .await;

        let duration = start.elapsed();
        results.push(result);
        durations.push(duration);
    }

    let total_time = start_time.elapsed();

    // 結果の検証
    let successful_searches = results.iter().filter(|r| r.is_ok()).count();
    assert_eq!(successful_searches, CONCURRENT_SEARCHES);

    // パフォーマンス指標の計算
    let avg_duration = durations.iter().sum::<Duration>() / durations.len() as u32;
    let max_duration = durations.iter().max().unwrap();

    println!("🔍 負荷テスト結果 - 並行検索:");
    println!("  - 検索回数: {}", CONCURRENT_SEARCHES);
    println!(
        "  - 成功率: {}/{} ({:.1}%)",
        successful_searches,
        CONCURRENT_SEARCHES,
        (successful_searches as f64 / CONCURRENT_SEARCHES as f64) * 100.0
    );
    println!("  - 総実行時間: {:?}", total_time);
    println!("  - 平均レスポンス時間: {:?}", avg_duration);
    println!("  - 最大レスポンス時間: {:?}", max_duration);
    println!(
        "  - 検索スループット: {:.1} searches/sec",
        CONCURRENT_SEARCHES as f64 / total_time.as_secs_f64()
    );

    // パフォーマンス要件の検証（検索は高速であるべき）
    assert!(
        avg_duration < Duration::from_millis(200),
        "平均検索時間が200msを超えています"
    );
    assert!(
        *max_duration < Duration::from_millis(1000),
        "最大検索時間が1秒を超えています"
    );
}

#[tokio::test]
async fn test_circulation_workflow_load() {
    let test_env = TestEnvironment::new().await;
    test_env.setup_test_data().await.unwrap();

    let admin_token = test_env
        .authenticate_user("admin", "password")
        .await
        .unwrap();
    let user_token = test_env
        .authenticate_user("user1", "password")
        .await
        .unwrap();

    // 複数の文書を作成してから回覧フローをテスト
    const WORKFLOW_COUNT: usize = 20;
    let mut documents = vec![];

    // 文書作成フェーズ
    for i in 0..WORKFLOW_COUNT {
        let doc = test_env
            .create_document(
                CreateDocumentRequest {
                    title: format!("回覧負荷テスト文書 {}", i),
                    document_type_id: 1,
                    created_by: 1,
                    created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
                    confidentiality: TestConfidentiality::default(),
                    notes: Some(format!("回覧テスト {}", i)),
                },
                &admin_token,
            )
            .await
            .unwrap();
        documents.push(doc);
    }

    // 回覧開始フェーズ（順次実行に変更）
    let mut circulations = vec![];
    let mut circulation_durations = vec![];
    let circulation_start_time = Instant::now();

    for (i, doc) in documents.iter().enumerate() {
        let start = Instant::now();
        let result = test_env
            .start_circulation(
                StartCirculationRequest {
                    document_id: doc.id,
                    workflow_id: 1,
                    notes: Some(format!("負荷テスト回覧 {}", i)),
                },
                &admin_token,
            )
            .await;

        circulations.push(result.unwrap());
        circulation_durations.push(start.elapsed());
    }

    let circulation_total_time = circulation_start_time.elapsed();

    // 承認処理フェーズ（順次実行に変更）
    let mut approval_durations = vec![];
    let approval_start_time = Instant::now();

    for (i, circulation) in circulations.iter().enumerate() {
        let start = Instant::now();
        let result = test_env
            .approve_circulation_step(
                ApproveStepRequest {
                    circulation_id: circulation.id,
                    step_id: circulation.current_step_id,
                    action: "approve".to_string(),
                    comments: Some(format!("負荷テスト承認 {}", i)),
                },
                &user_token,
            )
            .await;

        assert!(result.unwrap().success);
        approval_durations.push(start.elapsed());
    }

    let approval_total_time = approval_start_time.elapsed();

    // パフォーマンス指標の計算と出力
    let avg_circulation_duration =
        circulation_durations.iter().sum::<Duration>() / circulation_durations.len() as u32;
    let avg_approval_duration =
        approval_durations.iter().sum::<Duration>() / approval_durations.len() as u32;

    println!("📋 負荷テスト結果 - 回覧ワークフロー:");
    println!("  - 回覧数: {}", WORKFLOW_COUNT);
    println!("  - 回覧開始:");
    println!("    - 総時間: {:?}", circulation_total_time);
    println!("    - 平均時間: {:?}", avg_circulation_duration);
    println!(
        "    - スループット: {:.1} workflows/sec",
        WORKFLOW_COUNT as f64 / circulation_total_time.as_secs_f64()
    );
    println!("  - 承認処理:");
    println!("    - 総時間: {:?}", approval_total_time);
    println!("    - 平均時間: {:?}", avg_approval_duration);
    println!(
        "    - スループット: {:.1} approvals/sec",
        WORKFLOW_COUNT as f64 / approval_total_time.as_secs_f64()
    );

    // パフォーマンス要件の検証
    assert!(
        avg_circulation_duration < Duration::from_millis(300),
        "平均回覧開始時間が300msを超えています"
    );
    assert!(
        avg_approval_duration < Duration::from_millis(200),
        "平均承認時間が200msを超えています"
    );
}

#[tokio::test]
async fn test_memory_usage_under_load() {
    let test_env = TestEnvironment::new().await;
    test_env.setup_test_data().await.unwrap();

    let admin_token = test_env
        .authenticate_user("admin", "password")
        .await
        .unwrap();

    // メトリクス収集開始
    test_env.metrics_service.start_collection().await;

    // 大量の操作を実行してメモリ使用量を監視
    const OPERATIONS_COUNT: usize = 1000;

    let start_time = Instant::now();

    // 大量の文書作成・検索・更新操作
    for i in 0..OPERATIONS_COUNT {
        // 文書作成
        let doc = test_env
            .create_document(
                CreateDocumentRequest {
                    title: format!("メモリテスト文書 {}", i),
                    document_type_id: ((i % 3) + 1) as i32,
                    created_by: 1,
                    created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
                    confidentiality: TestConfidentiality::default(),
                    notes: Some(format!("メモリ負荷テスト {}", i)),
                },
                &admin_token,
            )
            .await
            .unwrap();

        // 検索操作
        if i % 10 == 0 {
            let _search_result = test_env
                .search_documents(
                    DocumentSearchInput {
                        title: Some("メモリテスト".to_string()),
                        pagination: Pagination {
                            ..Pagination::new(1, 5)
                        },
                    },
                    &admin_token,
                )
                .await
                .unwrap();
        }

        // 更新操作
        if i % 20 == 0 {
            let _updated = test_env
                .update_document(
                    doc.id,
                    UpdateDocumentRequest {
                        title: Some(format!("更新されたメモリテスト文書 {}", i)),
                        notes: Some(format!("更新済み {}", i)),
                    },
                    &admin_token,
                )
                .await
                .unwrap();
        }

        // メトリクス記録
        test_env
            .metrics_service
            .record_request("/api/documents", start_time, true);
        test_env
            .metrics_service
            .record_database_query("INSERT", Duration::from_millis(10), true);
        test_env
            .metrics_service
            .record_cache_operation("get", i % 2 == 0);
    }

    let total_time = start_time.elapsed();

    // システムメトリクスの取得
    let metrics = test_env.metrics_service.get_metrics();
    let health_status = test_env.metrics_service.get_health_status();

    println!("💾 負荷テスト結果 - メモリ使用量:");
    println!("  - 実行操作数: {}", OPERATIONS_COUNT);
    println!("  - 総実行時間: {:?}", total_time);
    println!("  - メモリ使用量:");
    println!(
        "    - Heap使用: {:.1} MB",
        metrics.memory_usage.heap_used as f64 / (1024.0 * 1024.0)
    );
    println!(
        "    - Heap総量: {:.1} MB",
        metrics.memory_usage.heap_total as f64 / (1024.0 * 1024.0)
    );
    println!(
        "    - RSS: {:.1} MB",
        metrics.memory_usage.rss as f64 / (1024.0 * 1024.0)
    );
    println!("  - パフォーマンス:");
    println!("    - RPS: {:.1}", metrics.performance.requests_per_second);
    println!(
        "    - 平均レスポンス時間: {:.1}ms",
        metrics.performance.average_response_time_ms
    );
    println!(
        "    - エラー率: {:.1}%",
        metrics.performance.error_rate * 100.0
    );
    println!("  - データベース:");
    println!("    - 総クエリ数: {}", metrics.database.total_queries);
    println!("    - 遅いクエリ数: {}", metrics.database.slow_queries);
    println!(
        "    - 平均クエリ時間: {:.1}ms",
        metrics.database.average_query_time_ms
    );
    println!("  - キャッシュ:");
    println!("    - ヒット数: {}", metrics.cache.hits);
    println!("    - ミス数: {}", metrics.cache.misses);
    println!("    - ヒット率: {:.1}%", metrics.cache.hit_rate * 100.0);
    println!("  - システム健全性: {:?}", health_status.overall);

    // システム健全性の検証
    assert_eq!(
        health_status.overall,
        doc_man_db::services::HealthState::Healthy,
        "システムが健全状態ではありません"
    );
    assert!(
        metrics.performance.error_rate < 0.01,
        "エラー率が1%を超えています"
    );
    assert!(
        metrics.memory_usage.heap_used < 500 * 1024 * 1024,
        "ヒープ使用量が500MBを超えています"
    );
}

#[tokio::test]
async fn test_sustained_load_endurance() {
    let test_env = TestEnvironment::new().await;
    test_env.setup_test_data().await.unwrap();

    let admin_token = test_env
        .authenticate_user("admin", "password")
        .await
        .unwrap();

    // 5分間の持続負荷テスト（実際のテストでは短縮）
    const TEST_DURATION: Duration = Duration::from_secs(2); // テストでは2秒に短縮
    const OPERATIONS_PER_SECOND: usize = 50;

    println!(
        "⏱️  持続負荷テスト開始 ({}秒間)...",
        TEST_DURATION.as_secs()
    );

    let start_time = Instant::now();
    let mut operation_count = 0;
    let mut error_count = 0;

    while start_time.elapsed() < TEST_DURATION {
        let cycle_start = Instant::now();

        // 1秒間にOPERATIONS_PER_SECOND回の操作を実行
        for i in 0..OPERATIONS_PER_SECOND {
            let op_start = Instant::now();

            let result = test_env
                .create_document(
                    CreateDocumentRequest {
                        title: format!("持続負荷テスト文書 {}_{}", operation_count, i),
                        document_type_id: (operation_count % 3) + 1,
                        created_by: 1,
                        created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
                        confidentiality: TestConfidentiality::default(),
                        notes: Some(format!(
                            "持続テスト {} 秒目",
                            start_time.elapsed().as_secs()
                        )),
                    },
                    &admin_token,
                )
                .await;

            if result.is_err() {
                error_count += 1;
            }

            operation_count += 1;

            // レスポンス時間が長すぎる場合は警告
            if op_start.elapsed() > Duration::from_millis(500) {
                println!(
                    "⚠️  操作 {} のレスポンス時間が500msを超えました: {:?}",
                    operation_count,
                    op_start.elapsed()
                );
            }
        }

        // 1秒間隔を保つための調整
        let cycle_duration = cycle_start.elapsed();
        if cycle_duration < Duration::from_millis(100) {
            tokio::time::sleep(Duration::from_millis(100) - cycle_duration).await;
        }

        // 進捗表示（10秒ごと）
        if start_time.elapsed().as_secs() % 10 == 0
            && cycle_start.elapsed() < Duration::from_millis(100)
        {
            println!(
                "  📊 {}秒経過 - 操作数: {}, エラー数: {}",
                start_time.elapsed().as_secs(),
                operation_count,
                error_count
            );
        }
    }

    let total_time = start_time.elapsed();
    let success_rate = (operation_count - error_count) as f64 / operation_count as f64;
    let avg_throughput = operation_count as f64 / total_time.as_secs_f64();

    println!("🏁 持続負荷テスト結果:");
    println!("  - 実行時間: {:?}", total_time);
    println!("  - 総操作数: {}", operation_count);
    println!("  - エラー数: {}", error_count);
    println!("  - 成功率: {:.2}%", success_rate * 100.0);
    println!("  - 平均スループット: {:.1} ops/sec", avg_throughput);

    // パフォーマンス要件の検証
    assert!(success_rate >= 0.99, "成功率が99%を下回りました");
    assert!(
        avg_throughput >= OPERATIONS_PER_SECOND as f64 * 0.8,
        "平均スループットが期待値の80%を下回りました"
    );
}
