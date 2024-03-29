use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};

const PG_HOST: &str = "localhost";
const PG_ROOT_DB: &str = "postgres";
const PG_ROOT_USER: &str = "postgres";
const PG_ROOT_PWD: &str = "postgres";

pub type Db = Pool<Postgres>;

pub async fn init_db() -> Result<Db, sqlx::Error> {
    new_db_pool(PG_HOST, PG_ROOT_DB, PG_ROOT_USER, PG_ROOT_PWD, 1).await;
}

async fn new_db_pool(
    host: &str,
    db: &str,
    user: &str,
    pwd: &str,
    max_con: u32,
) -> Result<Db, sqlx::Error> {
    let con_string = format!("postgres://{}:{}@{}/{}", user, pwd, host, db);
    PgPoolOptions::new()
        .max_connections(max_con)
        .connect_timeout(Duration::from_millis(500))
        .connect(&con_string)
        .await
}

#[cfg(test)]
mod tests {
    use super::init_db;

    #[tokio::test]
    async fn model_db_init_db() -> Result<(), Box<dyn std::error::Error>> {
        Ok(());
    }
}
