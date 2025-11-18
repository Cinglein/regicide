use crate::{ActorId, Error};
use axum::{
    extract::{FromRef, State},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use tokio::sync::RwLock;

pub const ACTOR_LIST_PATH: &str = "/lobbies";

#[derive(Clone, Debug, Default, FromRef)]
pub struct ActorList(Arc<RwLock<Vec<(ActorId, u8)>>>);

impl ActorList {
    pub async fn read(&self) -> Vec<(ActorId, u8)> {
        self.0.read().await.clone()
    }
    pub fn write(&self, list: Vec<(ActorId, u8)>) {
        let mut lock = self.0.blocking_write();
        *lock = list;
    }
}

#[utoipa::path(
    get,
    path = ACTOR_LIST_PATH,
    params(),
    responses(
        (status = 200, description = "Actor", body = [(String, u8)]),
        (status = 500, description = "Internal server error", body = String)
    )
)]
pub async fn get_actor_list(
    State(actor_list): State<ActorList>,
) -> Result<impl IntoResponse, Error> {
    let res = actor_list.read().await;
    Ok(Json(res))
}
