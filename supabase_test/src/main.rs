use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, FromRow};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Post {
    id: Uuid,
    title: String,
    content: Option<String>,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct NewPost {
    title: String,
    content: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // 環境変数の読み込み
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // データベース接続プールの作成
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Connected to Supabase PostgreSQL!");

    // 新しい投稿の追加
    let new_post = NewPost {
        title: "Hello from Rust".to_string(),
        content: Some("This is my first post from Rust to Supabase!".to_string()),
    };

    let post = sqlx::query_as::<_, Post>(
        "INSERT INTO posts (title, content) VALUES ($1, $2) RETURNING id, title, content, created_at"
    )
    .bind(&new_post.title)
    .bind(&new_post.content)
    .fetch_one(&pool)
    .await?;

    println!("Created post: {:?}", post);

    // 全ての投稿を取得
    let posts = sqlx::query_as::<_, Post>("SELECT id, title, content, created_at FROM posts")
        .fetch_all(&pool)
        .await?;

    println!("All posts:");
    for post in posts {
        println!("- {:?}", post);
    }

    Ok(())
}