use crate::{ActorId, UserId};
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashMap, fmt::Debug};
use ts_rs::TS;

pub trait Action:
    Clone + Debug + Serialize + DeserializeOwned + TS + Send + Sync + 'static
{
    type Shared: Default;
    type User: Default;
    type Msg: Serialize + DeserializeOwned + TS + Send + Sync + 'static;
    fn can_join(shared: &Self::Shared, user: &HashMap<UserId, Self::User>) -> bool;
    fn update(
        self,
        shared: &mut Self::Shared,
        user: &mut HashMap<UserId, Self::User>,
        user_id: UserId,
    );
    fn join_msg(actor_id: ActorId) -> Self::Msg;
    fn msg(shared: &Self::Shared, user: &HashMap<UserId, Self::User>) -> Vec<(UserId, Self::Msg)>;
}
