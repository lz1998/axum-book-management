use axum::http::StatusCode;
use axum::Json;

use crate::database::book;
use crate::handler::auth::Claims;
use crate::handler::idl::*;

pub async fn create_book(
    user: Claims,
    Json(req): Json<CreateBookRequest>,
) -> Result<Json<CreateBookResponse>, StatusCode> {
    book::create(&req.name, &user.username)
        .await
        .map(|_| Json(CreateBookResponse { success: true }))
        .map_err(|err| {
            tracing::error!("failed to create book, {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub async fn search_book(
    _user: Claims,
    Json(req): Json<SearchBookRequest>,
) -> Result<Json<SearchBookResponse>, StatusCode> {
    book::search(&req.query)
        .await
        .map(|books| {
            Json(SearchBookResponse {
                books: books
                    .into_iter()
                    .map(|book| Book {
                        id: book.id,
                        name: book.name,
                        operator: book.operator,
                        created_at: book.created_at.timestamp() as i32,
                        updated_at: book.updated_at.timestamp() as i32,
                    })
                    .collect(),
            })
        })
        .map_err(|err| {
            tracing::error!("failed to search book, {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub async fn update_book(
    user: Claims,
    Json(req): Json<UpdateBookRequest>,
) -> Result<Json<UpdateBookResponse>, StatusCode> {
    book::update(req.id, &req.name, &user.username)
        .await
        .map(|_| Json(UpdateBookResponse { success: true }))
        .map_err(|err| {
            tracing::error!("failed to update book, {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub async fn delete_book(
    _user: Claims,
    Json(req): Json<DeleteBookRequest>,
) -> Result<Json<DeleteBookResponse>, StatusCode> {
    book::delete(req.id)
        .await
        .map(|_| Json(DeleteBookResponse { success: true }))
        .map_err(|err| {
            tracing::error!("failed to delete book, {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
