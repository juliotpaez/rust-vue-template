use std::sync::Arc;

use serde::{Deserialize, Serialize};

lazy_static! {
    static ref PONG_STRING: Arc<String> = Arc::new("pong".to_string());
    static ref OK_STRING: Arc<String> = Arc::new("ok".to_string());
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum WsMessage {
    #[serde(rename = "req")]
    Request(WsRequest),

    #[serde(rename = "not")]
    Notification(WsNotification),

    #[serde(rename = "res")]
    Response(WsResponse),

    #[serde(rename = "err")]
    Error(WsError),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct WsRequest {
    pub id: Arc<String>,

    #[serde(flatten)]
    pub method: WsRequestMethod,
}

impl WsRequest {
    pub fn new(id: Arc<String>, method: WsRequestMethod) -> WsRequest {
        WsRequest {
            id,
            method,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct WsNotification {
    #[serde(flatten)]
    pub method: WsNotificationMethod,
}

impl WsNotification {
    pub fn new(method: WsNotificationMethod) -> WsMessage {
        WsMessage::Notification(WsNotification {
            method,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct WsResponse {
    pub id: Arc<String>,
    pub result: WsResponseResult,
}

impl WsResponse {
    pub fn pong_response(id: Arc<String>) -> WsResponse {
        WsResponse {
            id,
            result: WsResponseResult::Text(PONG_STRING.clone()),
        }
    }

    pub fn ok_response(id: Arc<String>) -> WsResponse {
        WsResponse {
            id,
            result: WsResponseResult::Text(OK_STRING.clone()),
        }
    }

    pub fn text_response(id: Arc<String>, message: Arc<String>) -> WsResponse {
        WsResponse {
            id,
            result: WsResponseResult::Text(message),
        }
    }

    pub fn text_response_string(id: Arc<String>, message: String) -> WsResponse {
        WsResponse {
            id,
            result: WsResponseResult::Text(Arc::new(message)),
        }
    }

    pub fn response_from(id: Arc<String>, result: WsResponseResult) -> WsResponse {
        WsResponse {
            id,
            result,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct WsError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Arc<String>>,
    pub eid: WsErrorId,
    pub message: Arc<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WsErrorId {
    IncorrectInput,
}

impl WsError {
    pub fn new(id: Arc<String>, eid: WsErrorId, message: Arc<String>) -> WsMessage {
        WsMessage::Error(WsError {
            id: Some(id),
            eid,
            message,
        })
    }

    pub fn new_no_id(eid: WsErrorId, message: Arc<String>) -> WsMessage {
        WsMessage::Error(WsError {
            id: None,
            eid,
            message,
        })
    }

    pub fn new_string(id: Arc<String>, eid: WsErrorId, message: String) -> WsMessage {
        WsMessage::Error(WsError {
            id: Some(id),
            eid,
            message: Arc::new(message),
        })
    }

    pub fn new_no_id_string(eid: WsErrorId, message: String) -> WsMessage {
        WsMessage::Error(WsError {
            id: None,
            eid,
            message: Arc::new(message),
        })
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "method", content = "params")]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum WsRequestMethod {
    Ping,
    Echo(Arc<String>),
    Shutdown,
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "method", content = "params")]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum WsNotificationMethod {
    AskMe(Arc<String>),
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum WsResponseResult {
    Text(Arc<String>),
}