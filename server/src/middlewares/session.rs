
use std::str::FromStr;
use mongodb::Collection;
use tower_cookies::Cookies;
use bson::{doc, oid::ObjectId};

use axum::{
    middleware::Next,
    extract::{State, Request},
    response::Response as AxumResponse,
};

use crate::{

    config::state::ApiState, 
    models::user::UserModel,
    
    services::{
        authentication::{is_token, new_session_token}, cookies::new_cookie, jwt::decode_jwt
    },

    responses::{
        Response,
        ApiResponse as ApiError,
    },
};

pub async fn session_validation(cookies: Cookies, State(state): ApiState, 
    mut req: Request, next: Next) -> Result<AxumResponse, ApiError> {

    let mut token = cookies.get("token").map(|cookie| cookie.value().to_string());

    let payload_id = match &token {
        
        Some(token) => decode_jwt(&token, &state.jwt_secret)?,
        
        None => {
            
            let refresh_token = match cookies.get("refresh") {
                Some(refresh_token) => refresh_token.value().to_string(),
                None => return Err(Response::UNAUTHORIZED)
            };

            let new_token = new_session_token(&refresh_token, &state.db, &state.jwt_secret).await?;
            let new_session_cookie = new_cookie("token", new_token.clone(), time::Duration::minutes(2));

            let _ = cookies.add(new_session_cookie);

            let user_id = decode_jwt(&new_token, &state.jwt_secret)?;

            token = Some(new_token);

            user_id
        }
    };

    let _ = is_token("exp_tokens", &state.db, &token.clone().unwrap()).await?;

    let users: Collection<UserModel> = state.db.collection("users");

    let id = ObjectId::from_str(&payload_id).unwrap();

    let filter = doc! { "_id": id };

    let user = users.find_one(filter, None).await
        .map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
    ;

    match user {
        
        Some(user) => {
            req.extensions_mut().insert(user);
            req.extensions_mut().insert(token.unwrap());
        },
        
        None => return Err(Response::UNAUTHORIZED)
    }

    Ok(next.run(req).await)
}