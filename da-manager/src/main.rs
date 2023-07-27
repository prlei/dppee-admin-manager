use axum::Router;

use da_manager::{CONTEXT, controller, init_context};
use da_manager::entity::config::ApplicationConfig;

#[tokio::main]
async fn main() {
    init_context().await;
    let config = CONTEXT.get::<ApplicationConfig>();
    let server = format!(
        "{}:{}",
        config.server().host(),
        config.server().port()
    );
    let app = Router::new()
        .nest("/api", controller::sys_contorller::init_router()
        );
    axum::Server::bind(&server.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
