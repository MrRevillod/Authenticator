
use super::ApiResponse;

pub const SUCCESS: ApiResponse = ApiResponse::Standard(200, "Exito");

pub const PASSWORD_RESET_SUCCESS: ApiResponse = ApiResponse::Standard(200, "Se ha actualizado tu contraseña.");

pub const PASSWORD_RESET_REQUEST: ApiResponse = ApiResponse::Standard(200, "Se ha enviado un correo con las instrucciones para recuperar tu contraseña.");

pub const REGISTER_SUCCESS:
    ApiResponse = ApiResponse::Standard(201, "Se ha registrado tu cuenta, verifica tu dirección decorreo para validarla")
;

pub const LOGOUT_SUCCESS:
    ApiResponse = ApiResponse::Standard(200, "Sesión cerrada")
;

pub const VALIDATION_SUCCESS:
    ApiResponse = ApiResponse::Standard(200, "Validación exitosa")
;
