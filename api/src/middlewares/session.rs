
use mongodb::bson::doc;
use tower_cookies::Cookies;

use axum::{
    middleware::Next,
    extract::{State, Request},
    response::Response as AxumResponse,
};

use crate::{
    config::state::{ApiState, JWT_SECRET}, 
    responses::{ApiResponse as ApiError, Response}, 
    services::{
        jwt::*,
        cookies::new_cookie, 
        user::{find_user, oid_from_str} 
    }
};

pub async fn session_validation(cookies: Cookies, State(state): ApiState, 
    mut req: Request, next: Next) -> Result<AxumResponse, ApiError> {

    let mut token = cookies.get("token").map(|cookie| cookie.value().to_string());

    let user_id = match &token {
        
        Some(token) => decode_jwt(&token, &JWT_SECRET)?.id,
        
        None => {
            
            let refresh_token = match cookies.get("refresh") {
                Some(refresh_token) => refresh_token.value().to_string(),
                None => return Err(Response::UNAUTHORIZED)
            };

            let (new_token, user_id) = new_session_token(&state.db, &refresh_token).await?;
            let session_cookie = new_cookie("token", Some(&new_token), "SESSION");

            cookies.add(session_cookie);

            token = Some(new_token);
            user_id
        }
    };

    check_token_exp(&state.db, &token.clone().unwrap()).await?;

    let id = oid_from_str(&user_id)?;
    let user = find_user(&state.db, doc! {"_id": id}).await;

    if let Err(_) = user {
        return Err(Response::UNAUTHORIZED)
    }

    req.extensions_mut().insert(user.unwrap());
    req.extensions_mut().insert(token.unwrap());

    Ok(next.run(req).await)
}