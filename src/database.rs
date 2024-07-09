use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tracing::{debug, info};

use crate::repo::{messages, tasks};

const DEFAULT_POOL_SIZE: u32 = 5;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("`{0}` is not set")]
    DatabaseUrlNotSet(String),
    #[error(transparent)]
    Migrate(#[from] sqlx::migrate::MigrateError),
}

/// Create a new database pool.
///
/// # Errors
///
/// Will return an error if the database URL can't be read, or if the connection to the database
/// can't be established.
pub async fn new_pool(var_prefix: Option<&str>) -> Result<Pool<Postgres>> {
    let pool_size = get_pool_size(var_prefix);
    let database_url = get_database_url(var_prefix)?;

    info!("Connecting to a database with a pool size of {}", pool_size);

    Ok(PgPoolOptions::new()
        .max_connections(pool_size)
        .connect(&database_url)
        .await?)
}

/// Prepare the database by running migrations and cleaning up after possible previous termination.
///
/// # Errors
///
/// Will return an error if the migrations can't be run or if there was a problem while cleaning up
/// after possible previous termination.
pub async fn prepare(pool: &Pool<Postgres>) -> Result<()> {
    debug!("Running migrations");
    sqlx::migrate!("db/migrations")
        .run(pool)
        .await
        .map_err(Error::Migrate)?;

    debug!("Cleaning up after possible previous termination");

    // TODO: continue writing the messages that were writing before the termination
    messages::transition_all(
        pool,
        crate::types::messages::Status::Writing,
        crate::types::messages::Status::Failed,
    )
    .await?;
    tasks::transition_all(
        pool,
        crate::types::tasks::Status::InProgress,
        crate::types::tasks::Status::ToDo,
    )
    .await?;

    Ok(())
}

fn get_pool_size(var_prefix: Option<&str>) -> u32 {
    let pool_size_var = var_prefix.map_or("POOL_SIZE".to_string(), |prefix| format!("{}_POOL_SIZE", prefix));

    match std::env::var(pool_size_var) {
        Ok(pool_size) => pool_size,
        Err(_) => "".to_string(),
    }.parse().unwrap_or(DEFAULT_POOL_SIZE)
}

fn get_database_url(var_prefix: Option<&str>) -> Result<String> {
    let database_url_var = var_prefix.map_or("DATABASE_URL".to_string(), |prefix| format!("{}_DATABASE_URL", prefix));
    
    Ok(std::env::var(&database_url_var).map_err(|_| Error::DatabaseUrlNotSet(database_url_var))?)
}
