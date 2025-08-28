use crate::batch::BatchType;

#[test]
fn test_batch_types_enum() {
    let batch_types = [
        BatchType::FileCheck,
        BatchType::AdSync,
        BatchType::DataCleanup,
        BatchType::DocumentBackup,
        BatchType::SystemMaintenance,
    ];
    assert_eq!(batch_types.len(), 5);
    assert!(batch_types.contains(&BatchType::FileCheck));
}

#[test]
fn test_batch_schedule_keys() {
    let mut schedules = std::collections::HashMap::new();
    schedules.insert("file_check".to_string(), "毎月1日 9:00".to_string());
    schedules.insert("ad_sync".to_string(), "毎週月曜日 6:00".to_string());

    assert!(schedules.contains_key("file_check"));
    assert!(schedules.contains_key("ad_sync"));
}
