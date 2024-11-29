use std::fmt;

use serde::{Deserialize, Serialize};
use sqlx::any::AnyRow;
use sqlx::{FromRow, Row};

#[derive(Clone, Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub is_admin: bool,
}

impl<'r> FromRow<'r, AnyRow> for User {
    fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
        let id: i32 = row.try_get("id")?;
        let uuid: String = row.try_get("uuid")?;
        let username: String = row.try_get("username")?;
        let password: String = row.try_get("password")?;
        let email: String = row.try_get("email")?;
        let is_admin: bool = row.try_get("is_admin")?;

        Ok(Self { id, uuid, username, password, email, is_admin })
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserRegister {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl UserRegister {
    pub fn set(&self, key: &str, value: String) -> Result<Self, String> {
        let mut clone = self.clone();
        match key {
            "username" => clone.username = value,
            "password" => clone.password = value,
            "email" => clone.email = value,
            _ => return Err(format!("Invalid key: {}", key)),
        }
        Ok(clone)
    }
}

impl fmt::Display for UserRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "UserRegister {{ username: {}, password: {}, email: {} }}",
            self.username, self.password, self.email
        )
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

impl UserLogin {
    pub fn set(&self, key: &str, value: String) -> Result<Self, String> {
        let mut clone = self.clone();
        match key {
            "username" => clone.username = value,
            "password" => clone.password = value,
            _ => return Err(format!("Invalid key: {}", key)),
        }
        Ok(clone)
    }
}

impl fmt::Display for UserLogin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UserLogin {{ username: {}, password: {} }}", self.username, self.password)
    }
}

#[derive(Clone, Debug, Default, Deserialize, FromRow, PartialEq, Serialize)]
pub struct UserInformation {
    pub uuid: String,
    pub username: String,
    pub email: String,
    pub is_admin: bool,
}

impl fmt::Display for UserInformation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "UserInformation {{ uuid: {}, username: {}, email: {}, is_admin: {} }}",
            self.uuid, self.username, self.email, self.is_admin
        )
    }
}

impl UserInformation {
    pub fn from_user(user: User) -> Self {
        Self {
            uuid: user.uuid,
            username: user.username,
            email: user.email,
            is_admin: user.is_admin,
        }
    }

    pub fn default() -> Self {
        Self {
            uuid: String::new(),
            username: String::new(),
            email: String::new(),
            is_admin: false,
        }
    }
}
