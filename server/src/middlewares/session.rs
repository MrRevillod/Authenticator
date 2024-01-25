
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
        jwt::decode_jwt,
        authentication::is_token
    },

    responses::{
        Response,
        ApiResponse as ApiError,
    },
};

pub async fn session_validation(cookies: Cookies, State(state): ApiState, 
    mut req: Request, next: Next) -> Result<AxumResponse, ApiError> {

    let token = match cookies.get("token") {
        Some(token) => token.value().to_string(),
        None => return Err(Response::UNAUTHORIZED)
    };

    let payload_id = decode_jwt(&token, &state.jwt_secret)?;

    let _ = is_token("exp_tokens", &state.db, &token).await?;

    let users: Collection<UserModel> = state.db.collection("users");
    let id = ObjectId::from_str(&payload_id).unwrap();
    
    let filter = doc! { "_id": id };

    let user = users.find_one(filter, None).await
        .map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
    ;

    match user {
        
        Some(user) => {
            req.extensions_mut().insert(user);
            req.extensions_mut().insert(token);
        },
        
        None => return Err(Response::UNAUTHORIZED)
    }

    Ok(next.run(req).await)
}