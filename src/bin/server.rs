use std::net::SocketAddr;

use axum::{routing::post, Router};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = Router::new()
        .nest(
            "/user",
            Router::new()
                .route("/register", post(book_management::handler::user::register))
                .route("/login", post(book_management::handler::user::login)),
        )
        .nest(
            "/book",
            Router::new()
                .route("/create", post(book_management::handler::book::create_book))
                .route("/search", post(book_management::handler::book::search_book))
                .route("/update", post(book_management::handler::book::update_book))
                .route("/delete", post(book_management::handler::book::delete_book)),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
