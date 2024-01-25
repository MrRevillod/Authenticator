
use super::ApiResponse;

pub const SUCCESS: ApiResponse = ApiResponse::Standard(200, "Exito");

pub const LOGIN_SUCCESS:
    ApiResponse = ApiResponse::Standard(200, "Inicio de sesión exitoso")
;

pub const REGISTER_SUCCESS:
    ApiResponse = ApiResponse::Standard(201, "Tu cuenta ha registrada, revisa tu correo para validarla")
;

pub const LOGOUT_SUCCESS:
    ApiResponse = ApiResponse::Standard(200, "Cierre de sesión exitoso")
;

pub const VALIDATION_SUCCESS:
    ApiResponse = ApiResponse::Standard(200, "Validación exitosa")
;
