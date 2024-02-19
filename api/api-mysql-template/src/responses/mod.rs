
pub mod success;
pub mod error;

use error::ApiError;
use core::result::Result;

// ApiResult => services return type

pub type ApiResult<T> = ApiResponse<T, ApiError>;

// ApiResponse => controllers return type

pub type ApiResponse<T, U> = Result<T, U>;
