
use uuid::Uuid;
use bcrypt::hash;
use sqlx::MySqlPool;

use crate::models::user::*;
use crate::utils::jwt::sign_jwt;
use crate::responses::ApiResult;
use crate::responses::error::ApiError;

pub async fn check_user_exists(db: &MySqlPool, 
    username: &String, email: &String ) -> ApiResult<bool> {
    
    let count = sqlx::query!(
        r#"SELECT COUNT(*) as count FROM User WHERE username = ? OR email = ?"#,
        username,
        email
    )
    .fetch_one(db)
    .await
    .map_err(|_| return ApiError::UnexpectedError);

    match count {
        Ok(record) => Ok(record.count > 0),
        Err(_) => Err(ApiError::UnexpectedError)
    }
}

pub async fn save_user(db: &MySqlPool, body: &PublicUserData) -> ApiResult<String> {

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
    .await
    .map_err(|_| return ApiError::UnexpectedError);

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
    .map_err(|_| return ApiError::UnexpectedError)?;

    let validated = SqlxBool::from(user.validated);
    let secret = format!("{}{:?}", &secret, validated.as_bool());
    
    let token_exp = chrono::Utc::now() + chrono::Duration::days(60);
    let token = sign_jwt(&user_id, &secret, token_exp)?;

    Ok(format!("http://localhost:4000/auth/validate/{}/{}", user_id, token))
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
    .map_err(|_| return ApiError::UnexpectedError)?;

    Ok(())
}
