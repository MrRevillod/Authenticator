
use super::ApiResponse;

pub const RESOURCE_NOT_FOUND:
    ApiResponse = ApiResponse::Standard(404, "Recursos no encontrado")
;

pub const EXPIRED: 
    ApiResponse = ApiResponse::Standard(401, "Solicitud inválida o expirada")
;

pub const UNAUTHORIZED:
    ApiResponse = ApiResponse::Standard(401, "No autorizado")
;

pub const USER_ALREADY_EXISTS:
    ApiResponse = ApiResponse::Standard(409, "Ya existe un usuario registrado con estos datos")
;

pub const BAD_REQUEST:
    ApiResponse = ApiResponse::Standard(400, "Solicitud inválida")
;

pub const INVALID_CREDENTIALS:
    ApiResponse = ApiResponse::Standard(401, "Email o contraseña incorrectos")
;

pub const ACCOUNT_NOT_VALIDATED:
    ApiResponse = ApiResponse::Standard(401, "Tu cuenta no está validada")
;

pub const INTERNAL_SERVER_ERROR: 
    ApiResponse = ApiResponse::Standard(500, "Error interno del servidor")
;