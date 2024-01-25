
## Endpoints

### Autenticación

Iniciar sesión en la aplicación

- [*] POST /auth/login 

Registrar un usuario en la aplicación

- [*] POST /auth/register

Cerrar sesión en la aplicación

- [*] POST /auth/logout

Refrescar el token de acceso

- [ ] POST /auth/refresh

### Usuarios

Obtener información de un usuario

- [] GET /users/:id

Obtener información del perfil de la sesión actual

- [] GET /users/profile

Actualizar información del perfil de la sesión actual

- [] PATCH /users/profile

Eliminar cuenta de la sesión actual

- [] DELETE /users/profile

Obtener información de los grupos de un usuario

- [] GET /users/groups

### Grupos

Obtener información de un grupo por su id

- [] GET /groups/:id

Crear un grupo

- [] POST /groups

Actualizar información de un grupo por su id

- [] PATCH /groups/:id

Eliminar un grupo por su id

- [] DELETE /groups/:id

Listar miembros de un grupo por su id

- [] GET /groups/:id/members

