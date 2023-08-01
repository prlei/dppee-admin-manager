use axum::Router;

use da_manager::{controller, get_server, init_context};

#[tokio::main]
async fn main() {
    init_context().await;
    let server = get_server().await;
    let app = Router::new()
        .nest("/api", controller::api_router()
        );
    axum::Server::bind(&server.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
