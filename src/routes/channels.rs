use http::{header::AUTHORIZATION, HeaderMap};
use serde::{Deserialize, Serialize};
use socketioxide::{
    extract::{Data, SocketRef, State},
    layer::SocketIoLayer,
    SocketIo,
};
use tracing::trace;

use crate::state::AppState;

#[derive(Deserialize, Serialize)]
struct SubscribeEvent {
    channel: String,
    auth: Auth,
}

#[derive(Deserialize, Serialize)]
struct Auth {
    #[serde(with = "http_serde::header_map")]
    headers: HeaderMap,
}

#[derive(Deserialize, Serialize)]
struct UnsubscribeEvent {
    channel: String,
    auth: Auth,
}

#[derive(Debug, Deserialize, Serialize)]
struct ClientEvent {
    channel: String,
    event: String,
    data: serde_json::Value,
}

pub async fn channels(state: &AppState) -> anyhow::Result<SocketIoLayer> {
    let (layer, io) = SocketIo::builder()
        .with_state(state.to_owned())
        .build_layer();

    io.ns("/", |socket: SocketRef| {
        socket.on(
            "subscribe",
            |socket: SocketRef, Data(data): Data<SubscribeEvent>, State(state): State<AppState>| async move {
                if is_authorized(&data.auth, &state) {
                    socket.join(data.channel.clone());
                    trace!("{} subscribed to channel: {}", socket.id, &data.channel);
                }
            },
        );

        socket.on(
            "unsubscribe",
            |socket: SocketRef, Data(data): Data<UnsubscribeEvent>, State(state): State<AppState>| async move {
                if is_authorized(&data.auth, &state) {
                    socket.leave(data.channel.clone());
                    trace!("{} unsubscribed from channel: {}", socket.id, &data.channel);
                }
            },
        );

        socket.on(
            "client event",
            |socket: SocketRef, Data(data): Data<ClientEvent>| async move {
                trace!(
                    "Received client event: {} {} {}",
                    &data.event,
                    &data.channel,
                    &data.data.to_string().chars().take(50).collect::<String>()
                );

                trace!("Socket currently in rooms: {:?}", socket.rooms());

                if let Err(_) = socket
                    .to(data.channel.clone())
                    .emit(data.event, &[data.channel, data.data.to_string()])
                    .await
                {
                    trace!("Couldn't send client event.");
                } else {
                    trace!("Sent client event.")
                }
            },
        );

        socket.on_disconnect(|_socket: SocketRef| async move {
            trace!("User left.");
        });
    });

    Ok(layer)
}

fn is_authorized(auth: &Auth, state: &AppState) -> bool {
    if let Some(bearer_header) = auth.headers.get(AUTHORIZATION) {
        if let Ok(bearer_str) = bearer_header.to_str() {
            let mut split = bearer_str.split_whitespace().take(2);

            if let Some(bearer_key) = split.next() {
                if bearer_key.to_lowercase() == "bearer" {
                    if let Some(bearer_value) = split.next() {
                        return bearer_value == state.config.auth.api_key;
                    }
                }
            }
        }
    }

    false
}
