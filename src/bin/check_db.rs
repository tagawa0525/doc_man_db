use sqlx::{Row, sqlite::SqlitePool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite://./data/dev.db";
    let pool = SqlitePool::connect(database_url).await?;

    // Check tables
    let tables = sqlx::query("SELECT name FROM sqlite_master WHERE type='table';")
        .fetch_all(&pool)
        .await?;

    println!("Tables in database:");
    for table in tables {
        let name: String = table.get("name");
        println!("- {}", name);
    }

    // Check documents table
    let count = sqlx::query("SELECT COUNT(*) as count FROM documents;")
        .fetch_one(&pool)
        .await?;

    let doc_count: i64 = count.get("count");
    println!("\nDocuments count: {}", doc_count);

    // Show all documents
    let docs = sqlx::query("SELECT id, number, title FROM documents ORDER BY id;")
        .fetch_all(&pool)
        .await?;

    println!("\nDocuments:");
    for doc in docs {
        let id: i32 = doc.get("id");
        let number: String = doc.get("number");
        let title: String = doc.get("title");
        println!("{}: {} - {}", id, number, title);
    }

    Ok(())
}
