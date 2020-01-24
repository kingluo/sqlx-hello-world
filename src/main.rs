use sqlx::Connect;
use sqlx::{Executor, Row};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn =
        sqlx::postgres::PgConnection::connect("postgres://tmp:tmp@localhost/tmp").await?;
    let _ = conn
        .send(
            r#"
CREATE TEMPORARY TABLE foo (id serial PRIMARY KEY, name text);
INSERT INTO foo (name) VALUES ('hello'), ('world');
            "#,
        )
        .await?;
    let row = sqlx::query("SELECT * FROM foo ORDER by id")
        .fetch_one(&mut conn)
        .await?;
    let id: i32 = row.get("id");
    let name: String = row.get("name");
    assert_eq!(id, 1);
    assert_eq!(name, "hello");
    Ok(())
}
