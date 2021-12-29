use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct RegisterResponse {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct LoginResponse {
    pub success: bool,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Book {
    pub id: i32,
    pub name: String,
    pub operator: String,
    pub created_at: i32,
    pub updated_at: i32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct CreateBookRequest {
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct CreateBookResponse {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct SearchBookRequest {
    pub query: String,
}
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct SearchBookResponse {
    pub books: Vec<Book>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct UpdateBookRequest {
    pub id: i32,
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct UpdateBookResponse {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct DeleteBookRequest {
    pub id: i32,
}
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct DeleteBookResponse {
    pub success: bool,
}
