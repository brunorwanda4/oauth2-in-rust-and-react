use std::sync::Arc;


use libs::db::Db;
use routers::all_routers::all_routers;

mod routers;
mod libs;
mod models;
mod errors;
mod handlers;

pub struct AppState {
    db : Db,
}

#[tokio::main]
async fn main() {
    let db = Db::init().await.unwrap();
    let mc = Arc::new(AppState {db : db});

    let all_routes = all_routers(mc);
    // build our application with a single route
    let app = all_routes;

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3404").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}