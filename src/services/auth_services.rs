
use uuid::Uuid;
use bcrypt::hash;
use sqlx::MySqlPool;

use crate::utils::jwt_utils::sign_jwt;
use crate::utils::types::{ApiResult, ApiError};

use crate::models::user_models::*;
use crate::models::auth_models::RegisterSchema;

pub async fn check_user_exists(db: &MySqlPool, 
    username: &String, email: &String ) -> ApiResult<bool> {
    
    let count = sqlx::query!(
        r#"SELECT COUNT(*) as count FROM User WHERE username = ? OR email = ?"#,
        username,
        email
    )
    .fetch_one(db)
    .await;

    match count {
        Ok(record) => Ok(record.count > 0),
        Err(_) => Err(ApiError::UnexpectedError)
    }
}

pub async fn save_user(db: &MySqlPool, body: &RegisterSchema) -> ApiResult<String> {

    let uuid = Uuid::new_v4().to_string();

    let password = match hash(&body.password, 6) {
        Ok(password) => password,
        Err(_) => return Err(ApiError::UnexpectedError),
    };

    let _ = sqlx::query!(
        r#"
            INSERT INTO User 
            (uuid, username, email, password, role, validated) 
            VALUES (?, ?, ?, ?, ?, ?)
        "#,
        &uuid.to_string(),
        &body.username,
        &body.email,
        &password,
        "USER_ROLE".to_string(),
        false
    )
    .execute(db)
    .await;

    Ok(uuid)
}

pub async fn create_validation_url(db: &MySqlPool, user_id: &String, 
    secret: &String) -> ApiResult<String> {

    let user = sqlx::query!(
        r#"SELECT validated FROM User WHERE uuid = ?"#,
        &user_id
    )
    .fetch_one(db)
    .await
    .map_err(|_| ApiError::UserNotFound)?;

    let validated = SqlxBool::from(user.validated);

    let secret = format!("{}{:?}", &secret, validated.as_bool());
    let token = sign_jwt(&user_id, &secret).await;

    if let Err(_) = token {
        return Err(ApiError::UnexpectedError);
    }

    Ok(format!("http://localhost:4000/auth/validate/{}/{}", user_id, token.unwrap()))
}


pub async fn save_expired_token(db: &MySqlPool, user_id: &String,
    token: &String) -> ApiResult<()> {

    let token_id = Uuid::new_v4().to_string();

    let _ = sqlx::query!(
        r#"
            INSERT INTO ExpiredToken 
            (token_id, token, userId) 
            VALUES (?, ?, ?)
        "#,
        &token_id,
        token,
        user_id
    )
    .execute(db)
    .await
    .map_err(|_e| ApiError::UnexpectedError)?;

    Ok(())
}
