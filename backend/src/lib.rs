use axum::{extract::FromRef, routing::get, Router};
use kanal::Sender;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod action;
mod actor;
mod card;
mod deck;
mod error;
mod list;
mod phase;
mod state;
mod ws;

pub use action::*;
pub use actor::*;
pub use card::*;
pub use deck::*;
pub use error::*;
pub use list::*;
pub use phase::*;
pub use state::*;
pub use ws::*;

const JOIN_BOUND: usize = 1024;

#[derive(Clone, FromRef)]
pub struct AppState {
    send_join: Sender<JoinReq<RegicideAction>>,
    actor_list: Arc<RwLock<Vec<ActorId>>>,
}

#[derive(OpenApi)]
#[openapi(paths(ws_handler))]
pub struct ApiDoc;

pub async fn serve() {
    let dir = "frontend/out";
    let static_service =
        ServeDir::new(dir).not_found_service(ServeFile::new(format!("{dir}/404.html")));

    let actor_list = Arc::new(RwLock::new(Vec::new()));
    let (send_join, recv_join) = kanal::bounded(JOIN_BOUND);
    let state = AppState {
        send_join,
        actor_list: Arc::clone(&actor_list),
    };
    std::thread::spawn(move || {
        actor_loop(recv_join, actor_list);
    });

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route(WS_HANDLER_PATH, get(ws_handler::<RegicideAction>))
        .route(ACTOR_LIST_PATH, get(get_actor_list))
        .fallback_service(static_service)
        .with_state(state)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
