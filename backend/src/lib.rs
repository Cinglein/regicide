use actor::{ActorList, JoinReq, actor_loop};
use axum::{Router, extract::FromRef, routing::get};
use game::RegicideAction;
use kanal::Sender;
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod error;
mod list;
mod tracing_setup;
mod ws;

pub use error::*;
pub use list::*;
pub use ws::*;

const JOIN_BOUND: usize = 1024;

#[derive(Clone, FromRef)]
pub struct AppState {
    send_join: Sender<JoinReq<RegicideAction>>,
    actor_list: ActorList,
}

#[derive(OpenApi)]
#[openapi(paths(ws_handler))]
pub struct ApiDoc;

pub async fn serve() {
    tracing_setup::init_tracing();

    let dir = "frontend/out";
    let bind_addr = "0.0.0.0:3000";

    tracing::info!(
        dir = %dir,
        bind_addr = %bind_addr,
        "Starting Regicide server"
    );

    let static_service =
        ServeDir::new(dir).not_found_service(ServeFile::new(format!("{dir}/404.html")));

    let actor_list = ActorList::default();
    let (send_join, recv_join) = kanal::bounded(JOIN_BOUND);
    let state = AppState {
        send_join,
        actor_list: actor_list.clone(),
    };

    tracing::info!("Spawning actor system thread");
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

    tracing::info!("Binding to {}", bind_addr);
    let listener = tokio::net::TcpListener::bind(bind_addr).await.unwrap();

    tracing::info!("Server ready, listening on {}", bind_addr);
    axum::serve(listener, app).await.unwrap();
}
