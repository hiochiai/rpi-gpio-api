use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Error {
    title: String,
    status: u16,
    detail: String,
    instance: String,
}

macro_rules! instance {
    () => {{
        let caller_location = std::panic::Location::caller();
        format!("{}:{}", caller_location.file(), caller_location.line())
    }};
}

impl Error {
    #[allow(dead_code)]
    #[track_caller]
    pub fn new_invalid_json(err: String) -> Self {
        Error::new(
            StatusCode::BAD_REQUEST,
            format!("invalid json: {}", err),
            instance!(),
        )
    }

    #[allow(dead_code)]
    #[track_caller]
    pub fn new_value_does_not_follow_the_rule(value: String, key: String, err: String) -> Self {
        Error::new(
            StatusCode::BAD_REQUEST,
            format!(
                "'{}' does not follow the rule for '{}': {}",
                value, key, err
            ),
            instance!(),
        )
    }

    #[track_caller]
    pub fn new_not_found(id: String, err: String) -> Self {
        Error::new(
            StatusCode::NOT_FOUND,
            format!("'{}' is not found: {}", id, err),
            instance!(),
        )
    }

    #[track_caller]
    pub fn new_internal_server_error(err: String) -> Self {
        Error::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("unexpected error: {}", err),
            instance!(),
        )
    }

    fn new(status: StatusCode, detail: String, instance: String) -> Self {
        Self {
            title: status
                .canonical_reason()
                .unwrap_or("<unknown status code>")
                .to_string(),
            status: status.as_u16(),
            detail,
            instance,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (
            StatusCode::from_u16(self.status).unwrap(),
            serde_json::to_string_pretty(&self).unwrap(),
        )
            .into_response()
    }
}
