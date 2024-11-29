use std::env;
use std::time::Duration;

use once_cell::sync::OnceCell;
use sqlx::Pool;
use sqlx::any::{Any, AnyPoolOptions};

// Database pool singleton
static POOL: OnceCell<Pool<Any>> = OnceCell::new();

// Create database pool
pub async fn create_pool() {
    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(error) => panic!("Error with DATABASE_URL: {:?}", error),
    };
    init_pool(database_url).await;
}

// Initialize database pool
async fn init_pool(database_url: String) {
    // Install default drivers
    sqlx::any::install_default_drivers();

    // Create database pool
    let pool = match AnyPoolOptions::new()
        .max_connections(100)
        .idle_timeout(Some(Duration::from_secs(1)))
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Database pool created");
            pool
        }
        Err(error) => {
            panic!("Error creating database pool: {:?}", error);
        }
    };
    POOL.set(pool).unwrap();
}

pub fn get_pool() -> Pool<Any> {
    POOL.get().unwrap().to_owned()
}
