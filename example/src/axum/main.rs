use std::net::SocketAddr;
use rbatis::rbatis::Rbatis;
use rbatis::core::runtime::sync::Arc;
use axum::extract::Extension;
use example::BizActivity;
use rbatis::crud::CRUD;
use serde_json::Value;
use axum::AddExtensionLayer;
use axum::{
    handler::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};

//mysql driver url
pub const MYSQL_URL: &'static str = "mysql://root:123456@localhost:3306/test";

//handler
pub async fn handler(rb: Extension<Arc<Rbatis>>) -> Json<Value> {
    let v = rb.fetch_list::<BizActivity>().await.unwrap();
    Json(serde_json::json!(v))
}

#[tokio::main]
async fn main() {
    //log
    fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);

    log::info!("linking database...");
    let rb = Rbatis::new();
    rb.link(MYSQL_URL).await.expect("rbatis link database fail");
    let rb = Arc::new(rb);
    log::info!("linking database successful!");

    // build our application with a route
    let app = Router::new().route("/", get(handler))
        .layer(AddExtensionLayer::new(rb));
    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}", addr);
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}