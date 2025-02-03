use serde::{Deserialize, Serialize};
use socketioxide::{
    extract::{Data, SocketRef},
    layer::SocketIoLayer,
    SocketIo,
};
use tracing::trace;

#[derive(Deserialize, Serialize)]
struct SubscribeEvent {
    channel: String,
}

#[derive(Deserialize, Serialize)]
struct UnsubscribeEvent {
    channel: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ClientEvent {
    channel: String,
    event: String,
    data: serde_json::Value,
}

pub async fn channels() -> anyhow::Result<SocketIoLayer> {
    let (layer, io) = SocketIo::builder().build_layer();

    io.ns("/", |socket: SocketRef| {
        socket.on(
            "subscribe",
            |socket: SocketRef, Data(data): Data<SubscribeEvent>| async move {
                socket.join(data.channel.clone());

                trace!("{} subscribed to channel: {}", socket.id, &data.channel);
            },
        );

        socket.on(
            "unsubscribe",
            |socket: SocketRef, Data(data): Data<UnsubscribeEvent>| async move {
                socket.leave(data.channel.clone());

                trace!("{} unsubscribed from channel: {}", socket.id, &data.channel);
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
