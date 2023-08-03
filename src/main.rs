#[derive(Debug, sqlx::FromRow)]
struct User {
    pub id: i64,
    pub name: String,
}

use sqlx::mysql::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;

async fn print_users(pool: &MySqlPool) -> anyhow::Result<()> {
    println!("in fn print_users");
    let query = format!("select * from {}", "users");
    // let query = format!("select * from ?;");
    let users = sqlx::query_as::<_, User>(&query)
        // .bind("users")
        .fetch_all(pool)
        .await?;

    println!("{:?}", users.len());
    println!("{:?}", users);

    println!("out fn print_users");
    Ok(())
}

async fn insert_user(pool: &MySqlPool) -> anyhow::Result<()> {
    println!("in fn insert_user");
    let mut transaction = pool.begin().await?;
    let user = sqlx::query(
        r#"
insert into users (name)
values (?)
"#,
    )
    .bind("testtest")
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
    println!("{:?}", user);
    println!("out fn insert_user");
    Ok(())
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    println!("in fn main");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:password@localhost/try_sqlx_mysql")
        .await?;

    print_users(&pool).await?;

    insert_user(&pool).await?;

    print_users(&pool).await?;

    println!("out fn main");
    Ok(())
}
