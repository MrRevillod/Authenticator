
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use serde_json::json;
use serde::{Serialize, Deserialize};

use crate::models::user::UserSchema;

#[allow(non_snake_case)]
#[derive(Debug)]
pub enum ApiSuccess {
    Register(RegisterSuccess),
    Login(LoginSuccess),
    Logout,
    AccountValidated,
    GetUser(GetUserSuccess),
    GetUsers(GetUsersSuccess),
    UserUpdated,
    UserDeleted,
    ProfileUpdated,
    Refresh(LoginSuccess)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginSuccess {
    pub message: &'static str,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterSuccess {
    pub message: &'static str,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUserSuccess {
    pub user: UserSchema,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUsersSuccess {
    pub users: Vec<UserSchema>,
    pub results: usize,
}

impl IntoResponse for LoginSuccess {
    fn into_response(self) -> Response {
        let response = json!({
            "message": self.message,
            "token": self.token
        });
        
        (StatusCode::OK, Json(response)).into_response()
    }
}

impl IntoResponse for RegisterSuccess {
    fn into_response(self) -> Response {
        let response = json!({
            "message": self.message,
            "url": self.url
        });
        
        (StatusCode::CREATED, Json(response)).into_response()
    }
}

impl IntoResponse for GetUserSuccess {
    fn into_response(self) -> Response {
        let response = json!({
            "user": self.user
        });
        
        (StatusCode::OK, Json(response)).into_response()
    }
}

impl IntoResponse for GetUsersSuccess {
    fn into_response(self) -> Response {
        let response = json!({
            "users": self.users,
            "results": self.results
        });
        
        (StatusCode::OK, Json(response)).into_response()
    }
}

impl IntoResponse for ApiSuccess {
    fn into_response(self) -> Response {

        match self {
            ApiSuccess::Login(login_success_schema) => {
                login_success_schema.into_response()
            }

            ApiSuccess::Register(register_success_schema) => {
                register_success_schema.into_response()
            }

            ApiSuccess::Logout => {
                let response = json!({"message": "Logout success"});
                (StatusCode::OK, Json(response)).into_response()
            }

            ApiSuccess::AccountValidated => {
                let response = json!({"message": "Account validated"});
                (StatusCode::OK, Json(response)).into_response()
            }

            ApiSuccess::GetUser(get_user_success_schema) => {
                get_user_success_schema.into_response()
            }

            ApiSuccess::GetUsers(get_users_success_schema) => {
                get_users_success_schema.into_response()
            }

            ApiSuccess::UserUpdated => {
                let response = json!({"message": "User updated"});
                (StatusCode::OK, Json(response)).into_response()
            }

            ApiSuccess::UserDeleted => {
                let response = json!({"message": "User deleted"});
                (StatusCode::OK, Json(response)).into_response()
            }

            ApiSuccess::ProfileUpdated => {
                let response = json!({"message": "Profile updated"});
                (StatusCode::OK, Json(response)).into_response()
            }

            ApiSuccess::Refresh(refresh_success) => {
                refresh_success.into_response()
            }
        }
    }
}