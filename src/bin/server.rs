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

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
