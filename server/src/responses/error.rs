
use super::ApiResponse;

pub const RESOURCE_NOT_FOUND:
    ApiResponse = ApiResponse::Standard(404, "Recursos no encontrado")
;

pub const EXPIRED: 
    ApiResponse = ApiResponse::Standard(401, "Expirado")
;

pub const UNAUTHORIZED:
    ApiResponse = ApiResponse::Standard(401, "No autorizado")
;

pub const USER_ALREADY_EXISTS:
    ApiResponse = ApiResponse::Standard(400, "El usuario ya existe")
;

pub const INVALID_CREDENTIALS:
    ApiResponse = ApiResponse::Standard(401, "Credenciales inválidas")
;

pub const ACCOUNT_NOT_VALIDATED:
    ApiResponse = ApiResponse::Standard(401, "Cuenta no validada")
;

pub const INTERNAL_SERVER_ERROR: 
    ApiResponse = ApiResponse::Standard(500, "Error interno del servidor")
;