use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
    pub success: bool,
}

impl<T> IntoResponse for ResResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        serde_json::to_string(&self).unwrap().into_response()
    }
}

impl<T: Serialize> ResResponse<T> {
    pub fn new(code: i32, message: String, data: Option<T>, success: bool) -> ResResponse<T> {
        ResResponse {
            code: code,
            message: message,
            data: data,
            success: success,
        }
    }

    pub fn success(message: String, data: Option<T>) -> ResResponse<T> {
        ResResponse::new(200, message, data, true)
    }
}
