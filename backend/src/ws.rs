use crate::*;
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
use kanal::{AsyncReceiver, Receiver, Sender};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub const WS_HANDLER_PATH: &str = "/ws";

const MSG_BOUND: usize = 16;

#[derive(Serialize, Deserialize)]
#[serde(bound = "A: Serialize + DeserializeOwned")]
pub enum ClientMsg<A: Action> {
    Join {
        lobby: ActorId,
        client_token: UserId,
    },
    Action {
        action: A,
    },
}

struct ClientHandle<A: Action> {
    user_id: UserId,
    recv_server_msg: Receiver<ServerMsg>,
    send_action: Sender<(A, UserId)>,
}

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
    ws.on_upgrade(async move |mut socket| {
        let mut channels = None;
        while let Some(Ok(msg)) = socket.recv().await {
            if let Some(ClientMsg::<A>::Join {
                lobby,
                client_token,
            }) = msg
                .to_text()
                .ok()
                .and_then(|txt| serde_json::from_str(txt).ok())
            {
                // todo: add auth
                let user_id = client_token;
                let actor_id = Some(lobby);
                let (send_server_msg, recv_server_msg) = kanal::bounded(MSG_BOUND);
                let (send_sender, recv_sender) = kanal::bounded(MSG_BOUND);
                let join_req = JoinReq::<A>::Connect {
                    user_id,
                    actor_id,
                    send_server_msg,
                    send_sender,
                };
                match send_join.send(join_req) {
                    Ok(()) => match recv_sender.to_async().recv().await {
                        Ok(send_action) => {
                            channels = Some(ClientHandle {
                                user_id,
                                recv_server_msg,
                                send_action,
                            });
                        }
                        Err(_err) => (),
                    },
                    Err(_err) => (),
                }
            }
        }
        if let Some(ClientHandle {
            user_id,
            recv_server_msg,
            send_action,
        }) = channels
        {
            let (sender, receiver) = socket.split();
            let write_handle = tokio::spawn(write(sender, recv_server_msg.to_async()));
            let read_handle = tokio::spawn(read::<A>(receiver, user_id, send_action));
            let (write_res, read_res) = tokio::join!(write_handle, read_handle);
            if let Err(_err) = write_res {}
            if let Err(_err) = read_res {}
            if let Err(_err) = send_join.send(JoinReq::Disconnect { user_id }) {}
        }
    })
}

async fn read<A: Action>(
    mut recv: SplitStream<WebSocket>,
    user_id: UserId,
    send_action: Sender<(A, UserId)>,
) {
    while let Some(Ok(msg)) = recv.next().await {
        if let Some(ClientMsg::Action::<A> { action }) = msg
            .to_text()
            .ok()
            .and_then(|txt| serde_json::from_str(txt).ok())
        {
            if let Err(_err) = send_action.send((action, user_id)) {}
        }
    }
}

async fn write(mut send: SplitSink<WebSocket, Message>, recv: AsyncReceiver<ServerMsg>) {
    while let Ok(msg) = recv.recv().await {
        match serde_json::to_string(&msg) {
            Ok(txt) => if let Err(_err) = send.send(Message::Text(txt.into())).await {},
            Err(_err) => (),
        }
    }
}
