use crate::Action;
use arrayvec::ArrayString;
use kanal::{Receiver, Sender};
use std::{
    collections::HashMap,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;
use uuid::Uuid;

pub type UserId = ArrayString<32>;
pub type ActorId = Uuid;

const RECV_BOUND: usize = 128;
const TICK_MS: Duration = Duration::from_millis(10);

#[derive(Clone, Debug, Default)]
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

pub fn actor_loop<A: Action>(recv: Receiver<JoinReq<A>>, actor_list: ActorList) {
    let mut actors = ActorSystem::<A>::new(recv, actor_list);
    let mut next = Instant::now() + TICK_MS;
    loop {
        actors.update();
        let now = Instant::now();
        if now < next {
            thread::sleep(next - now);
        }
        next += TICK_MS;
    }
}

pub struct ActorSystem<A: Action> {
    recv: Receiver<JoinReq<A>>,
    actors: HashMap<ActorId, Actor<A>>,
    users: HashMap<UserId, UserHandle<A>>,
    actor_list: ActorList,
}

impl<A: Action> ActorSystem<A> {
    pub fn new(recv: Receiver<JoinReq<A>>, actor_list: ActorList) -> Self {
        Self {
            recv,
            actors: Default::default(),
            users: Default::default(),
            actor_list,
        }
    }
    fn update(&mut self) {
        self.actors.values_mut().for_each(|actor| {
            actor.update();
            let msgs = A::msg(&actor.shared, &actor.user);
            msgs.into_iter().for_each(|(user_id, msg)| {
                if let Some(send) = actor.server_msgs.get(&user_id)
                    && let Err(_err) = send.send(msg)
                {}
            });
        });
        while let Ok(Some(join)) = self.recv.try_recv() {
            self.join(join);
        }
    }
    fn update_list(&mut self) {
        let actor_list = self
            .actors
            .iter()
            .filter_map(|(id, a)| a.open().map(|i| (*id, i)))
            .collect();
        self.actor_list.write(actor_list);
    }
    fn join(&mut self, join: JoinReq<A>) {
        match join {
            JoinReq::Connect {
                user_id,
                actor_id,
                send_server_msg,
                send_sender,
            } => {
                if let Some(UserHandle {
                    connected,
                    actor_id,
                    send_action,
                }) = self.users.get_mut(&user_id)
                    && let Some(actor) = self.actors.get_mut(actor_id)
                {
                    match send_sender.send(send_action.clone()) {
                        Ok(()) => {
                            if let Some(send) = actor.server_msgs.get_mut(&user_id) {
                                match send_server_msg.send(A::join_msg(*actor_id)) {
                                    Ok(()) => {
                                        *send = send_server_msg;
                                        *connected = true;
                                    }
                                    Err(_err) => (),
                                }
                            }
                        }
                        Err(_err) => (),
                    }
                } else if let Some(actor) = actor_id.and_then(|id| self.actors.get_mut(&id))
                    && <A as Action>::can_join(&actor.shared, &actor.user)
                    && let Some(send_action) = actor
                        .user
                        .keys()
                        .find_map(|id| self.users.get(id).map(|h| h.send_action.clone()))
                {
                    match send_sender.send(send_action.clone()) {
                        Ok(()) => match send_server_msg.send(A::join_msg(actor_id.unwrap())) {
                            Ok(()) => {
                                self.users.insert(
                                    user_id,
                                    UserHandle {
                                        connected: true,
                                        actor_id: actor_id.unwrap(),
                                        send_action,
                                    },
                                );
                                actor.user.insert(user_id, Default::default());
                                actor.server_msgs.insert(user_id, send_server_msg);
                            }
                            Err(_err) => (),
                        },
                        Err(_err) => (),
                    }
                } else {
                    let (actor, send_action) = Actor::spawn(user_id, send_server_msg);
                    match send_sender.send(send_action.clone()) {
                        Ok(()) => {
                            let actor_id = Uuid::now_v7();
                            match actor
                                .server_msgs
                                .get(&user_id)
                                .unwrap()
                                .send(A::join_msg(actor_id))
                            {
                                Ok(()) => {
                                    self.actors.insert(actor_id, actor);
                                    self.users.insert(
                                        user_id,
                                        UserHandle {
                                            connected: true,
                                            actor_id,
                                            send_action,
                                        },
                                    );
                                    self.update_list();
                                }
                                Err(_err) => (),
                            }
                        }
                        Err(_err) => (),
                    }
                }
            }
            JoinReq::Disconnect { user_id } => {
                let actor_id = if let Some(handle) = self.users.get_mut(&user_id) {
                    handle.connected = false;
                    Some(handle.actor_id)
                } else {
                    None
                };
                if let Some(users) = actor_id.and_then(|id| {
                    self.actors
                        .get(&id)
                        .map(|a| a.user.keys().copied().collect::<Vec<_>>())
                }) && !users
                    .iter()
                    .filter_map(|id| self.users.get(id))
                    .any(|h| h.connected)
                {
                    users.iter().for_each(|id| {
                        self.users.remove(id);
                    });
                    self.actors.remove(&actor_id.unwrap());
                    self.update_list();
                }
            }
        }
    }
}

pub struct Actor<A: Action> {
    recv: Receiver<(A, UserId)>,
    shared: A::Shared,
    user: HashMap<UserId, A::User>,
    server_msgs: HashMap<UserId, Sender<A::Msg>>,
}

impl<A: Action> Actor<A> {
    fn open(&self) -> Option<u8> {
        A::can_join(&self.shared, &self.user).then_some(self.user.len() as u8)
    }
    fn spawn(user_id: UserId, send_server_msg: Sender<A::Msg>) -> (Self, Sender<(A, UserId)>) {
        let shared = Default::default();
        let mut user = HashMap::default();
        user.insert(user_id, Default::default());
        let mut server_msgs = HashMap::default();
        server_msgs.insert(user_id, send_server_msg);
        let (send, recv) = kanal::bounded(RECV_BOUND);
        (
            Self {
                recv,
                shared,
                user,
                server_msgs,
            },
            send,
        )
    }
    fn update(&mut self) {
        while let Ok(Some((msg, user_id))) = self.recv.try_recv() {
            Action::update(msg, &mut self.shared, &mut self.user, user_id);
        }
    }
}

pub struct UserHandle<A: Action> {
    connected: bool,
    actor_id: ActorId,
    send_action: Sender<(A, UserId)>,
}

pub enum JoinReq<A: Action> {
    Connect {
        user_id: UserId,
        actor_id: Option<ActorId>,
        send_server_msg: Sender<A::Msg>,
        send_sender: Sender<Sender<(A, UserId)>>,
    },
    Disconnect {
        user_id: UserId,
    },
}
