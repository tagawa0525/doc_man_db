use sqlx::{Row, sqlite::SqlitePool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite://./data/dev.db";
    let pool = SqlitePool::connect(database_url).await?;

    // Check document_types table
    let count = sqlx::query("SELECT COUNT(*) as count FROM document_types;")
        .fetch_one(&pool)
        .await?;

    let types_count: i64 = count.get("count");
    println!("Document types count: {}", types_count);

    // Show all document types
    let types = sqlx::query("SELECT id, name, prefix FROM document_types ORDER BY id;")
        .fetch_all(&pool)
        .await?;

    println!("\nDocument types:");
    for doc_type in types {
        let id: i32 = doc_type.get("id");
        let name: String = doc_type.get("name");
        let prefix: String = doc_type.get("prefix");
        println!("{}: {} ({})", id, name, prefix);
    }

    Ok(())
}
