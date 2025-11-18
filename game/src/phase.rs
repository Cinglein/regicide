use actor::UserId;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../frontend/src/bindings/")]
pub enum Phase {
    #[schema(value_type = String)]
    Play(#[ts(as = "String")] UserId),
    #[schema(value_type = String)]
    Jester(#[ts(as = "String")] UserId),
    #[schema(value_type = String)]
    Defend(#[ts(as = "String")] UserId),
    Victory,
    Defeat,
}
