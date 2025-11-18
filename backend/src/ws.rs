use crate::*;
use actor::{Action, UserId};
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use game::{ClientMsg, ServerMsg};
use kanal::{AsyncReceiver, Sender};
use tokio::sync::oneshot;
use tracing::{debug, error, info, warn};

pub const WS_HANDLER_PATH: &str = "/ws";

const MSG_BOUND: usize = 16;

#[utoipa::path(
    get,
    path = WS_HANDLER_PATH,
    params(),
    responses(
        (status = 101, description = "A WS stream of Message values", body = ServerMsg),
    )
)]
pub async fn ws_handler<A: Action>(
    ws: WebSocketUpgrade,
    State(send_join): State<Sender<JoinReq<A>>>,
) -> impl IntoResponse {
    ws.on_upgrade(async move |socket| {
        debug!("WebSocket connection opened");
        let (send_receiver, recv_receiver) = oneshot::channel();
        let (sender, receiver) = socket.split();
        let read_handle = tokio::spawn(read::<A>(receiver, send_join.clone(), send_receiver));
        let write_handle = tokio::spawn(write::<A>(sender, recv_receiver));
        let (write_res, read_res) = tokio::join!(write_handle, read_handle);
        let user_id = match read_res {
            Ok(Some(user_id)) => user_id,
            Ok(None) => {
                warn!("Read handler disconnected without receiving user_id");
                return;
            }
            Err(err) => {
                error!(error = %err, "Read handler failed");
                return;
            }
        };
        if let Err(err) = write_res {
            error!(%user_id, error = %err, "Write handler failed");
        }
        debug!(%user_id, "WebSocket closed, sending disconnect");
        if let Err(err) = send_join.send(JoinReq::Disconnect { user_id }) {
            error!(%user_id, error = %err, "Failed to send disconnect");
        }
    })
}

async fn read<A: Action>(
    mut recv: SplitStream<WebSocket>,
    send_join: Sender<JoinReq<A>>,
    send_receiver: oneshot::Sender<AsyncReceiver<A::Msg>>,
) -> Option<UserId> {
    match wait_join(&mut recv, send_join).await {
        Ok((user_id, send_action, recv_server_msg)) => {
            if let Err(_err) = send_receiver.send(recv_server_msg) {
                error!(%user_id, "Failed to send receiver");
                return Some(user_id);
            }
            while let Some(Ok(msg)) = recv.next().await {
                let bytes = msg.into_data();
                match postcard::from_bytes(&bytes) {
                    Ok(ClientMsg::Action::<A> { action }) => {
                        debug!(%user_id, "Received action from client");
                        if let Err(err) = send_action.send((action, user_id)) {
                            error!(%user_id, error = %err, "Failed to send action to actor");
                        }
                    }
                    Ok(_other) => {
                        warn!(%user_id, "Received unexpected message");
                    }
                    Err(err) => {
                        error!(error = %err, "Failed to deserialize action");
                    }
                };
            }
            Some(user_id)
        }
        Err(err) => {
            error!(error = %err, "Failed to join");
            None
        }
    }
}

async fn write<A: Action>(
    mut send: SplitSink<WebSocket, Message>,
    recv_receiver: oneshot::Receiver<AsyncReceiver<A::Msg>>,
) {
    match recv_receiver.await {
        Ok(recv) => {
            while let Ok(msg) = recv.recv().await {
                match postcard::to_stdvec(&msg) {
                    Ok(bytes) => {
                        if let Err(err) = send.send(Message::Binary(bytes.into())).await {
                            error!(error = %err, "Failed to send message to client");
                        }
                    }
                    Err(err) => {
                        error!(error = %err, "Failed to serialize server message");
                    }
                }
            }
        }
        Err(err) => {
            error!(error = %err, "Failed to receive ServerMsg receiver");
        }
    }
}

async fn wait_join<A: Action>(
    recv: &mut SplitStream<WebSocket>,
    send_join: Sender<JoinReq<A>>,
) -> Result<(UserId, Sender<(A, UserId)>, AsyncReceiver<A::Msg>), Error> {
    while let Some(Ok(msg)) = recv.next().await {
        let bytes = msg.into_data();
        match postcard::from_bytes(&bytes) {
            Ok(ClientMsg::Join::<A> {
                client_token,
                lobby,
            }) => {
                let user_id = client_token;
                info!(%user_id, lobby = ?lobby, "Client join request");
                let (send_server_msg, recv_server_msg) = kanal::bounded(MSG_BOUND);
                let (send_sender, recv_sender) = kanal::bounded(MSG_BOUND);
                let join_req = JoinReq::<A>::Connect {
                    user_id,
                    actor_id: lobby,
                    send_server_msg,
                    send_sender,
                };
                match send_join.send(join_req) {
                    Ok(()) => match recv_sender.to_async().recv().await {
                        Ok(send_action) => {
                            return Ok((user_id, send_action, recv_server_msg.to_async()));
                        }
                        Err(err) => {
                            error!(%user_id, error = %err, "Failed to receive action sender");
                        }
                    },
                    Err(err) => {
                        error!(%user_id, error = %err, "Failed to send join request");
                    }
                }
            }
            Ok(other) => {
                warn!("Received unexpected message: {other:?}");
            }
            Err(err) => {
                error!(error = %err, "Failed to deserialize action");
            }
        };
    }
    Err(Error::JoinError)
}
