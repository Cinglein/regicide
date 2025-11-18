use crate::Error;
use actor::ActorList;
use axum::{Json, extract::State, response::IntoResponse};

pub const ACTOR_LIST_PATH: &str = "/lobbies";

#[utoipa::path(
    get,
    path = ACTOR_LIST_PATH,
    params(),
    responses(
        (status = 200, description = "Actor", body = [(String, u8)]),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[tracing::instrument(skip(actor_list))]
pub async fn get_actor_list(
    State(actor_list): State<ActorList>,
) -> Result<impl IntoResponse, Error> {
    let res = actor_list.read().await;
    let lobby_count = res.len();

    tracing::debug!(lobby_count, "Returning lobby list");

    Ok(Json(res))
}
