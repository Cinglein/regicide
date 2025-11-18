use crate::{ActorId, Error};
use axum::{extract::State, response::IntoResponse, Json};
use std::sync::Arc;
use tokio::sync::RwLock;

pub const ACTOR_LIST_PATH: &str = "/lobbies";

#[utoipa::path(
    get,
    path = ACTOR_LIST_PATH,
    params(),
    responses(
        (status = 200, description = "Actor", body = [String]),
        (status = 500, description = "Internal server error", body = String)
    )
)]
pub async fn get_actor_list(
    State(actor_list): State<Arc<RwLock<Vec<ActorId>>>>,
) -> Result<impl IntoResponse, Error> {
    let res: Vec<ActorId> = actor_list.read().await.clone();
    Ok(Json(res))
}
