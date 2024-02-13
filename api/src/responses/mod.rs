
pub mod error;
pub mod success;

#[allow(non_snake_case)]
pub mod Response {
    pub use super::error::*;
    pub use super::success::*;
}

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

pub enum ApiResponse {
    
    /// `Standard` es una respuesta estándar.
    /// 
    /// # Parámetros
    /// 
    /// * `u16`: Código de estado HTTP.
    /// * `&'static str`: Mensaje de la respuesta.
    
    Standard(u16, &'static str),

    /// `DataResponse` es una respuesta que contiene datos.
    /// 
    /// # Parámetros
    /// 
    /// * `u16`: Código de estado HTTP.
    /// * `&'static str`: Mensaje de la respuesta.
    /// * `&'static str`: Tipo de los datos.
    /// * `Value`: Los datos de la respuesta.
    
    DataResponse(u16, &'static str, &'static str, Value),
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
        }
    }
}