// backend/src/db.rs

//! Database connection pool setup.

use std::time::Duration;

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

/// Pool settings shared by the app. Kept small for a single-instance MVP.
pub fn pool_options() -> PgPoolOptions {
    PgPoolOptions::new()
        .max_connections(5)
        // How long a request waits for a free connection (or a new one)
        // before giving up. Keeps /health/ready from hanging when the DB is down.
        .acquire_timeout(Duration::from_secs(3))
}

/// Connect to PostgreSQL. Fails if the database cannot be reached right now.
pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    pool_options().connect(database_url).await
}
