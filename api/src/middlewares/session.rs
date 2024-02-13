
use std::str::FromStr;
use tower_cookies::Cookies;

use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
};

use axum::{
    middleware::Next,
    extract::{State, Request},
    response::Response as AxumResponse,
};

use crate::{

    models::user::UserModel,
    
    services::{
        
        jwt::decode_jwt,
        cookies::new_cookie, 
        
        authentication::{
            is_exp_token, 
            new_session_token
        }, 
    },
    
    responses::{
        Response,
        ApiResponse as ApiError,
    },
    
    config::state::{ApiState, JWT_SECRET}, 
};

pub async fn session_validation(cookies: Cookies, State(state): ApiState, 
    mut req: Request, next: Next) -> Result<AxumResponse, ApiError> {

    let mut token = cookies.get("token").map(|cookie| cookie.value().to_string());

    let payload_id = match &token {
        
        Some(token) => decode_jwt(&token, &JWT_SECRET)?.id,
        
        None => {
            
            let refresh_token = match cookies.get("refresh") {
                Some(refresh_token) => refresh_token.value().to_string(),
                None => return Err(Response::UNAUTHORIZED)
            };

            let new_token = new_session_token(&refresh_token, &state.db).await?;
            let new_session_cookie = new_cookie("token", new_token.clone(), time::Duration::minutes(60));

            let _ = cookies.add(new_session_cookie);

            let payload = decode_jwt(&new_token, &JWT_SECRET)?;

            token = Some(new_token);
            payload.id
        }
    };

    let _ = is_exp_token(&token.clone().unwrap(), &state.db).await?;

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