use clap::{Parser, Subcommand};
use doc_man_db::seeds::{Environment, Seeder};
use sqlx::sqlite::SqlitePool;
use std::process;

#[derive(Parser)]
#[command(
    name = "seeds",
    about = "Database seeding tool for doc_man_db",
    version = "1.0.0"
)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Environment to use for seeding
    #[arg(short, long, default_value = "development")]
    env: String,

    /// Specific table to seed (if not specified, seeds all tables)
    #[arg(short, long)]
    table: Option<String>,

    /// Dry run mode - show what would be done without making changes
    #[arg(short = 'n', long)]
    dry_run: bool,

    /// Reset database before seeding (clears existing data)
    #[arg(short, long)]
    reset: bool,

    /// Database URL (defaults to ./data/dev.db)
    #[arg(long, default_value = "sqlite://./data/dev.db")]
    database_url: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Run seeding operation
    Run,

    /// List available seed files for the specified environment
    List,

    /// Clear data from specified table or all tables
    Clear {
        /// Specific table to clear
        #[arg(short, long)]
        table: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // 環境の解析
    let environment = match args.env.parse::<Environment>() {
        Ok(env) => env,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    // データベース接続
    let pool = match SqlitePool::connect(&args.database_url).await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            process::exit(1);
        }
    };

    let seeder = Seeder::new(pool);

    // コマンドの実行
    let result = match args.command.unwrap_or(Commands::Run) {
        Commands::Run => {
            run_seeding(&seeder, &environment, args.table, args.dry_run, args.reset).await
        }
        Commands::List => list_seeds(&seeder, &environment).await,
        Commands::Clear { table } => clear_data(&seeder, &environment, table, args.dry_run).await,
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

/// シードデータの投入を実行
async fn run_seeding(
    seeder: &Seeder,
    environment: &Environment,
    table: Option<String>,
    dry_run: bool,
    reset: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🌱 Doc Management Database Seeder");
    println!("Environment: {}", environment);

    if dry_run {
        println!("🔍 DRY RUN MODE - No changes will be made to the database");
    }

    if reset && !dry_run {
        println!("⚠️  RESET MODE - Existing data will be cleared");
    }

    match table {
        Some(table_name) => {
            println!("Seeding specific table: {}", table_name);
            seeder.seed_table(environment, &table_name, dry_run).await?;
        }
        None => {
            println!("Seeding all tables");
            seeder.seed_all(environment, dry_run, reset).await?;
        }
    }

    println!("✅ Seeding completed successfully!");
    Ok(())
}

/// 利用可能なシードファイルをリスト表示
async fn list_seeds(
    _seeder: &Seeder,
    environment: &Environment,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("📋 Available seed files for environment: {}", environment);

    let loader = doc_man_db::seeds::loader::SeedLoader::default();
    let tables = loader.list_available_tables(environment)?;

    if tables.is_empty() {
        println!("No seed files found for environment: {}", environment);
        return Ok(());
    }

    println!("Available tables:");
    for table in &tables {
        println!("  • {}", table);
    }

    println!("\nTotal: {} seed files", tables.len());
    Ok(())
}

/// データのクリア操作
async fn clear_data(
    seeder: &Seeder,
    environment: &Environment,
    table: Option<String>,
    dry_run: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if dry_run {
        println!("🔍 DRY RUN MODE - No data will actually be cleared");
    } else {
        println!("⚠️  WARNING: This will permanently delete data!");
    }

    match table {
        Some(table_name) => {
            println!("Clearing table: {}", table_name);
            seeder
                .clear_table(environment, &table_name, dry_run)
                .await?;
        }
        None => {
            println!("Clearing all tables");
            let loader = doc_man_db::seeds::loader::SeedLoader::default();
            let tables = loader.list_available_tables(environment)?;

            // 逆順でクリア（依存関係を考慮）
            for table_name in tables.iter().rev() {
                seeder.clear_table(environment, table_name, dry_run).await?;
            }
        }
    }

    if !dry_run {
        println!("✅ Data clearing completed!");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_args_parsing() {
        // デフォルト値のテスト
        let args = Args::parse_from(&["seeds"]);
        assert_eq!(args.env, "development");
        assert!(!args.dry_run);
        assert!(!args.reset);
        assert_eq!(args.database_url, "sqlite://./data/dev.db");
    }

    #[test]
    fn test_args_with_options() {
        let args = Args::parse_from(&[
            "seeds",
            "--env",
            "production",
            "--table",
            "employees",
            "--dry-run",
            "--reset",
            "--database-url",
            "sqlite://./test.db",
        ]);

        assert_eq!(args.env, "production");
        assert_eq!(args.table, Some("employees".to_string()));
        assert!(args.dry_run);
        assert!(args.reset);
        assert_eq!(args.database_url, "sqlite://./test.db");
    }

    #[test]
    fn test_environment_parsing() {
        assert_eq!(
            "development".parse::<Environment>().unwrap(),
            Environment::Development
        );
        assert_eq!(
            "dev".parse::<Environment>().unwrap(),
            Environment::Development
        );
        assert_eq!("test".parse::<Environment>().unwrap(), Environment::Test);
        assert_eq!(
            "production".parse::<Environment>().unwrap(),
            Environment::Production
        );
        assert_eq!(
            "prod".parse::<Environment>().unwrap(),
            Environment::Production
        );

        assert!("invalid".parse::<Environment>().is_err());
    }
}
