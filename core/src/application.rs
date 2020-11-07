use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};
use tokio::sync::oneshot::Sender;
use warp::filters::ws::Message;

use crate::network::ws::messages::{WsError, WsResponse};

pub type AppContextRef = Arc<Mutex<AppContext>>;

pub struct AppContext {
    // Allows to close the application.
    pub shutdown_trigger: Option<Sender<()>>,
    pub client: Option<AppClient>,
}

impl AppContext {
    pub fn new() -> AppContext {
        AppContext {
            shutdown_trigger: None,
            client: None,
        }
    }

    pub fn client_exists(&self) -> bool {
        return self.client.is_some();
    }

    pub fn shutdown(&mut self) {
        let shutdown_trigger = self.shutdown_trigger.take();
        if let Some(sender) = shutdown_trigger {
            sender.send(()).unwrap();
        }
    }
}

pub struct AppClient {
    pub origin: SocketAddr,
    pub sender: mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>,
    pub pending_answers: HashMap<Arc<String>, futures::channel::oneshot::Sender<Result<WsResponse, WsError>>>,
}