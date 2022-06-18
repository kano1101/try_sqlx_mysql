#[derive(Debug, sqlx::FromRow)]
struct User {
    pub id: i64,
    pub name: String,
}

use sqlx::mysql::MySqlPoolOptions;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:password@localhost/try_sqlx_mysql")
        .await?;

    let users = sqlx::query_as::<_, User>("select * from users")
        .fetch_all(&pool)
        .await?;

    println!("{:?}", users.len());
    println!("{:?}", users);

    Ok(())
}
