
pub mod error;
pub mod success;

#[allow(non_snake_case)]
pub mod Response {
    pub use super::error::*;
    pub use super::success::*;
}

use std::collections::HashMap;
use serde_json::{json, Value};

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response as AxumResponse}, 
};

// type for controllers / handlers responses
pub type HttpResponse = Result<ApiResponse, ApiResponse>;

// type for services results
pub type ApiResult<T> = Result<T, ApiResponse>;

// enum for base responses -> impl is in src/responses/templates.rs
pub enum ApiResponse {
    Standard(u16, &'static str),
    DataResponse(u16, &'static str, &'static str, Value),
    BadRequest(HashMap<&'static str, &'static str>)
}

impl IntoResponse for ApiResponse {

    fn into_response(self) -> AxumResponse {

        match self {

            ApiResponse::Standard(status, message) => {

                let code = StatusCode::from_u16(status).unwrap();
                
                let data = json!({
                    "status": status,
                    "message": message,
                });

                (code, Json(data)).into_response()
            }

            ApiResponse::DataResponse(status, message, data_name, data) => {

                let code = StatusCode::from_u16(status).unwrap();
                
                let data = json!({
                    "status": status,
                    "message": message,
                    data_name: data,
                });

                (code, Json(data)).into_response()
            }

            ApiResponse::BadRequest(errors) => {

                let code = StatusCode::BAD_REQUEST;
                
                let data = json!({
                    "status": 400,
                    "message": "Error de validación de datos",
                    "errors": errors,
                });

                (code, Json(data)).into_response()
            }
        }
    }
}