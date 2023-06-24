use axum::response::{IntoResponse, Json, Response};
use rppal::system::DeviceInfo;
use serde::Serialize;

use crate::api;

#[derive(Serialize, Debug)]
pub struct Device {
    model: String,
}

pub async fn get() -> Response {
    let device = match DeviceInfo::new() {
        Ok(device) => device,
        Err(e) => {
            return api::Error::new_internal_server_error(e.to_string()).into_response();
        }
    };

    Json(Device {
        model: device.model().to_string(),
    })
    .into_response()
}
