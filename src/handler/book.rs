use axum::Json;

use crate::database::book;
use crate::error::CustomResult;
use crate::handler::auth::Claims;
use crate::handler::idl::*;

pub async fn create_book(
    user: Claims,
    Json(req): Json<CreateBookRequest>,
) -> CustomResult<Json<CreateBookResponse>> {
    book::create(&req.name, &user.username)
        .await
        .map(|_| Json(CreateBookResponse { success: true }))
}

pub async fn search_book(
    _user: Claims,
    Json(req): Json<SearchBookRequest>,
) -> CustomResult<Json<SearchBookResponse>> {
    book::search(&req.query).await.map(|books| {
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
}

pub async fn update_book(
    user: Claims,
    Json(req): Json<UpdateBookRequest>,
) -> CustomResult<Json<UpdateBookResponse>> {
    book::update(req.id, &req.name, &user.username)
        .await
        .map(|_| Json(UpdateBookResponse { success: true }))
}

pub async fn delete_book(
    _user: Claims,
    Json(req): Json<DeleteBookRequest>,
) -> CustomResult<Json<DeleteBookResponse>> {
    book::delete(req.id)
        .await
        .map(|_| Json(DeleteBookResponse { success: true }))
}
