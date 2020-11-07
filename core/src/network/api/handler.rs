use std::convert::Infallible;
use std::error::Error;
use std::net::SocketAddr;

use serde::Serialize;
use uuid::Uuid;
use warp::{Rejection, Reply, reply::json};
use warp::http::StatusCode;

use crate::application::AppContextRef;
use crate::network::api::errors::{ConnectionAlreadyInUse, OriginAddressRequired};
use crate::network::ws::websocket_connection;

pub async fn version_handler() -> Result<impl Reply, Rejection> {
    Ok(json(&VersionResponse {
        name: env!("CARGO_PKG_NAME"),
        description: env!("CARGO_PKG_DESCRIPTION"),
        version: env!("CARGO_PKG_VERSION"),
        authors: env!("CARGO_PKG_AUTHORS"),
        homepage: env!("CARGO_PKG_HOMEPAGE"),
    }))
}

pub async fn ws_handler(ws: warp::ws::Ws, origin: Option<SocketAddr>, context_ref: AppContextRef) -> Result<impl Reply, Rejection> {
    // Prevent connection if there is already one.
    if context_ref.lock().await.client_exists() {
        Err(warp::reject::custom(ConnectionAlreadyInUse {}))
    } else {
        let origin = match origin {
            None => {
                return Err(warp::reject::custom(OriginAddressRequired {}));
            }
            Some(v) => v
        };

        let connection_id = Uuid::new_v4().to_hyphenated().to_string();
        Ok(ws.on_upgrade(move |socket| websocket_connection(socket, connection_id, origin, context_ref)))
    }
}

// This function receives a `Rejection` and tries to return a custom
// value, otherwise simply passes the rejection along.
pub(crate) async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(_e) = err.find::<ConnectionAlreadyInUse>() {
        code = StatusCode::BAD_REQUEST;
        message = "CONNECTION_ALREADY_IN_USE";
    } else if let Some(_e) = err.find::<OriginAddressRequired>() {
        code = StatusCode::BAD_REQUEST;
        message = "ORIGIN_ADDRESS_REQUIRED";
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        // This error happens if the body could not be deserialized correctly
        // We can use the cause to analyze the error and customize the error message
        message = match e.source() {
            Some(cause) => {
                if cause.to_string().contains("denom") {
                    "FIELD_ERROR: denom"
                } else {
                    "BAD_REQUEST"
                }
            }
            None => "BAD_REQUEST",
        };
        code = StatusCode::BAD_REQUEST;
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorResponse {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}

#[derive(Serialize, Debug)]
struct VersionResponse {
    name: &'static str,
    description: &'static str,
    version: &'static str,
    authors: &'static str,
    homepage: &'static str,
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    code: u16,
    message: String,
}