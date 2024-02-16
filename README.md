
# Workflow Aplicación Fullstack

### Arquitectura basada en Microservicios

![img](./arq_software.png)


### Api Endpoints

#### Autenticación

- **POST** /auth/login - Iniciar sesión
- **POST** /auth/register - Registrar usuario
- **POST** /auth/logout - Cerrar sesión
- **POST** /auth/validate - Validar sesión

- **POST** /auth/reset-password - Solicitar cambio de contraseña
- **POST** /auth/reset-password/:id/:token - validar url de cambio de contraseña
- **PATCH** /auth/reset-password/:id/:token - Actualizar contraseña

#### Account

- **PATCH** /account/:id - Actualizar perfil
- **GET** /account/update-email/:id/:token - Validar nuevo email 
- **DELETE** /account/:id - Eliminar cuenta
- **POST** /account/validate/:id/:token - Validar cuenta creada recientemente

#### Users

- **GET** /users/:id - Obtener información pública de un usuario

