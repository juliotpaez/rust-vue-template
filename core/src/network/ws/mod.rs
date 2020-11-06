use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

use futures::{FutureExt, StreamExt};
use serde::Deserialize;
use serde::export::Option::Some;
use serde_json::from_str;
use tokio::sync::mpsc;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

use crate::application::{AppClient, AppContextRef};
use crate::network::ws::errors::ClientDisconnectedError;
use crate::network::ws::messages::{WsError, WsErrorId, WsMessage, WsNotification, WsNotificationMethod, WsRequest, WsRequestMethod, WsResponse};

pub mod messages;
mod errors;

#[derive(Deserialize, Debug)]
pub struct TopicsRequest {
    topics: Vec<String>,
}

pub async fn websocket_connection(ws: WebSocket, connection_id: String, origin: SocketAddr, context_ref: AppContextRef) {
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    // Redirect messages.
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            error!("error sending websocket msg: {}", e);
        }
    }));

    // Add to context.
    {
        let mut context = context_ref.lock().await;
        context.client = Some(AppClient {
            origin,
            sender: client_sender,
            pending_answers: HashMap::new(),
        });
    }

    info!("Client {} connected", connection_id);

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                error!("Error while receiving ws message: {}", e);
                break;
            }
        };

        parse_client_message(&connection_id, msg, &context_ref).await;
    }

    // Clean up a bit of memory.
    context_ref.lock().await.client = None;

    info!("Client {} disconnected", connection_id);
}

async fn parse_client_message(_connection_id: &str, msg: Message, context_ref: &AppContextRef) {
    debug!("Received message: {:?}", msg);
    let message_str = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    let message: WsMessage = match from_str(message_str) {
        Ok(v) => v,
        Err(e) => {
            error!("Error while parsing message: {}", e);
            let _ = send_ws_message(WsMessage::Error(WsError::new_no_id_string(WsErrorId::IncorrectInput, e.to_string())), context_ref).await;
            return;
        }
    };

    match message {
        WsMessage::Request(request) => {
            let response: Result<WsResponse, WsError> = match request.method {
                WsRequestMethod::Ping => {
                    trace!("[METHOD] Ping");
                    Ok(WsResponse::pong_response(request.id))
                }
                WsRequestMethod::Echo(text) => {
                    trace!("[METHOD] Echo");
                    Ok(WsResponse::text_response(request.id, text))
                }
                WsRequestMethod::Shutdown => {
                    trace!("[METHOD] Shutdown");
                    let response = WsResponse::ok_response(request.id);
                    let _ = send_ws_message(WsMessage::Response(response), context_ref).await;

                    info!("Shutting down");

                    context_ref.lock().await.shutdown();
                    return;
                }
            };

            let message = match response {
                Ok(r) => WsMessage::Response(r),
                Err(e) => WsMessage::Error(e)
            };

            if let Err(e) = send_ws_message(message, context_ref).await {
                error!("Error while sending response: {}", e)
            }
        }
        WsMessage::Notification(notification) => {
            match notification.method {
                WsNotificationMethod::AskMe(text) => {
                    let connection_id = Arc::new(Uuid::new_v4().to_hyphenated().to_string());
                    match send_ws_request(WsRequest::new(connection_id, WsRequestMethod::Echo(text)), context_ref).await {
                        Ok(rx) => {
                            tokio::spawn(async move {
                                match rx.await {
                                    Ok(response) => {
                                        warn!("{:?}", response)
                                    }
                                    Err(e) => {
                                        error!("Ignoring request because it was cancelled: {}", e)
                                    }
                                }
                            });
                        }
                        Err(e) => {
                            error!("Error while sending request: {}", e)
                        }
                    }
                }
            };
        }

        WsMessage::Response(response) => {
            trace!("Received response: {:?}", response);

            let mut context = context_ref.lock().await;
            let client: &mut AppClient = if let Some(client) = &mut context.client {
                client
            } else {
                error!("Unhandled response: {:?}", response);
                return;
            };

            if let Some(tx) = client.pending_answers.remove(&response.id) {
                if let Err(response) = tx.send(Ok(response)) {
                    error!("Unhandled response because client is disconnected: {:?}", response);
                };
            } else {
                error!("Unhandled response: {:?}", response);
            }
        }
        WsMessage::Error(error) => {
            if let Some(id) = &error.id {
                trace!("Received error: {:?}", error);

                let mut context = context_ref.lock().await;
                let client: &mut AppClient = if let Some(client) = &mut context.client {
                    client
                } else {
                    error!("Unhandled error: {:?}", error);
                    return;
                };

                if let Some(tx) = client.pending_answers.remove(id) {
                    if let Err(error) = tx.send(Err(error)) {
                        error!("Unhandled error because client is disconnected: {:?}", error);
                    };
                } else {
                    error!("Unhandled error: {:?}", error);
                }
            } else {
                error!("Received error: {:?}", error);
            }
        }
    }
}

pub async fn send_ws_request(request: WsRequest, context_ref: &AppContextRef) -> Result<futures::channel::oneshot::Receiver<Result<WsResponse, WsError>>, Box<dyn Error + Send>> {
    // Save sender.
    let (tx, rx) = futures::channel::oneshot::channel();
    let mut context = context_ref.lock().await;
    let connection_id = request.id.clone();
    if let Some(client) = &mut context.client {
        client.pending_answers.insert(connection_id.clone(), tx);
    } else {
        return Err(Box::new(ClientDisconnectedError {}));
    }

    // Drop locks.
    std::mem::drop(context);

    // Send request.
    if let Err(e) = send_ws_message(WsMessage::Request(request), context_ref).await {
        // Remove the receiver.
        let mut context = context_ref.lock().await;
        if let Some(client) = &mut context.client {
            client.pending_answers.remove(&connection_id);
        }

        return Err(e);
    }

    Ok(rx)
}

pub async fn send_ws_notification(notification: WsNotification, context_ref: &AppContextRef) -> Result<(), Box<dyn Error + Send>> {
    send_ws_message(WsMessage::Notification(notification), context_ref).await
}

async fn send_ws_message(message: WsMessage, context_ref: &AppContextRef) -> Result<(), Box<dyn Error + Send>> {
    let json_response = serde_json::to_string(&message).unwrap();
    let response_message = Ok(Message::text(json_response));

    let context = context_ref.lock().await;

    if let Some(client) = context.client.as_ref() {
        trace!("Sending message: {:?}", &message);

        match client.sender.send(response_message) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e))
        }
    } else {
        Err(Box::new(ClientDisconnectedError {}))
    }
}