use crate::RegicideAction;
use actor::{Action, ActorId, UserId};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS)]
#[serde(bound = "A: Serialize + DeserializeOwned")]
#[ts(concrete(A = RegicideAction))]
#[ts(export, export_to = "../../frontend/src/bindings/")]
pub enum ClientMsg<A: Action> {
    Join {
        #[ts(as = "Option<String>")]
        lobby: Option<ActorId>,
        #[ts(as = "String")]
        client_token: UserId,
    },
    Action {
        action: A,
    },
}
