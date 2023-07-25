pub mod models;

#[cfg(feature = "postgres")]
pub type DatabasePool = sqlx::PgPool;

#[cfg(feature = "mysql")]
pub type DatabasePool = sqlx::MySqlPool;

#[cfg(feature = "sqlite")]
pub type DatabasePool = sqlx::SqlitePool;
