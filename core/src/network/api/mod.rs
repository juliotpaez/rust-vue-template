use std::convert::Infallible;
use std::error::Error;
use std::net::SocketAddr;

use warp::{Filter, Future};

use crate::application::AppContextRef;
use crate::errors::EmptyError;

mod handler;
mod errors;

pub async fn init_api(addr: &SocketAddr, context: AppContextRef) -> Result<impl Future<Output = ()>, Box<dyn Error>> {
    // Create routes.
    let version_route = warp::path!("version").and_then(handler::version_handler);
    let ws_route = warp::path("ws").and(warp::ws()).and(warp::addr::remote()).and(with_context(context)).and_then(handler::ws_handler);
    let routes = version_route.or(ws_route).with(warp::cors().allow_any_origin()).recover(handler::handle_rejection);

    // Init server.
    let (addr, server) = match warp::serve(routes).try_bind_ephemeral((addr.ip(), addr.port())) {
        Ok(v) => v,
        Err(e) => {
            error!("Cannot connect API socket: {}", e);
            return Err(Box::new(EmptyError {}));
        }
    };

    info!("Listening on http://{}", addr);

    Ok(server)
}

fn with_context(context: AppContextRef) -> impl Filter<Extract = (AppContextRef, ), Error = Infallible> + Clone {
    warp::any().map(move || context.clone())
}