#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate tokio;

use std::env;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::sync::{Mutex, oneshot};

use crate::application::{AppContext, AppContextRef};
use crate::errors::EmptyError;

mod network;
mod errors;
mod application;
mod utils;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // Read environment variables.
    let addr: SocketAddr = env::var("ADDR")
        .unwrap_or_else(|_| "127.0.0.1:21012".to_string()).parse().unwrap_or_else(|_| {
        error!("A valid address was expected for the ADDR environment variable");
        panic!();
    });

    // Initiate context.
    let mut context = AppContext::new();
    let (sender, receiver) = oneshot::channel();
    context.shutdown_trigger = Some(sender);
    let context_ref: AppContextRef = Arc::new(Mutex::new(context));

    // Initiate server.
    let api_future = async {
        // Get the handler.
        let handler = match network::api::init_api(&addr, context_ref.clone()).await {
            Ok(v) => v,
            Err(err) => {
                error!("Error creating API: {}", err);
                panic!();
            }
        };

        // Wait for requests.
        handler.await;

        info!("API closed");
        Ok(())
    };

    let shutdown_future = async {
        receiver.await.ok();
        let result: Result<(), Box<dyn Error>> = Err(Box::new(EmptyError {}));
        result
    };

    match try_join!(api_future, shutdown_future) {
        Ok(_) => {}
        Err(_) => {}
    };

    info!("Good bye!");
}