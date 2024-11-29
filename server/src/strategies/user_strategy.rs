use std::env;

use bcrypt::{DEFAULT_COST, hash_with_salt};
use sqlx::any::{AnyQueryResult, AnyRow};
use uuid::Uuid;

use crate::pool::get_pool;
use crate::types::user::{User, UserRegister};

// Get all database users
pub async fn get_db_users() -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
            SELECT * FROM \"users\"
        "#,
    )
    .fetch_all(&get_pool())
    .await
}

// Get database user by identifier (username or email)
pub async fn get_db_user_by_identifier(identifier: String) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
            SELECT * FROM \"users\"
            WHERE username = $1 OR email = $1
        "#,
    )
    .bind(identifier)
    .fetch_one(&get_pool())
    .await
}

// Get database user by UUID
pub async fn get_db_user_by_uuid(uuid: String) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
            SELECT * FROM \"users\"
            WHERE uuid = $1
        "#,
    )
    .bind(uuid)
    .fetch_one(&get_pool())
    .await
}

// Insert database user to database
pub async fn insert_db_user(user_register: UserRegister) -> Result<User, sqlx::Error> {
    // Create uuid
    let id = Uuid::new_v4();

    // Generate salt and hash password
    let mut salt: [u8; 16] = [0; 16];
    salt.copy_from_slice(&env::var("PASSWORD_SALT").unwrap().as_bytes()[0..16]);
    let hashed_password =
        hash_with_salt(user_register.password, DEFAULT_COST, salt).unwrap().to_string();

    // Query database
    sqlx::query_as::<_, User>(
        r#"
            INSERT INTO \"users\" (uuid, username, password, email, is_admin)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
        "#,
    )
    .bind(id.to_string())
    .bind(user_register.username)
    .bind(hashed_password)
    .bind(user_register.email)
    .bind(false)
    .fetch_one(&get_pool())
    .await
}

// Update database user in database
pub async fn update_db_user(user: User) -> Result<AnyRow, sqlx::Error> {
    // Generate salt and hash password
    let mut salt: [u8; 16] = [0; 16];
    salt.copy_from_slice(&env::var("PASSWORD_SALT").unwrap().as_bytes()[0..16]);
    let hashed_password = hash_with_salt(user.password, DEFAULT_COST, salt).unwrap().to_string();

    // Query database
    sqlx::query(
        r#"
            UPDATE \"users\"
            SET uuid = $2, username = $3, password = $4, email = $5, is_admin = $6
            WHERE uuid = $1
            RETURNING *
        "#,
    )
    .bind(user.id)
    .bind(user.uuid)
    .bind(user.username)
    .bind(hashed_password)
    .bind(user.email)
    .bind(user.is_admin)
    .fetch_one(&get_pool())
    .await
}

// Delete database user from database by UUID
pub async fn delete_db_user_by_uuid(uuid: String) -> Result<AnyQueryResult, sqlx::Error> {
    sqlx::query(
        r#"
            DELETE FROM \"users\"
            WHERE uuid = $1
            RETURNING *
        "#,
    )
    .bind(uuid)
    .execute(&get_pool())
    .await
}
