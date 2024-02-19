
use mongodb::bson::{doc, oid::ObjectId};

use axum::{
    
    extract::{
        Path, Request, State
    }, 
    
    middleware::Next, 
    response::Response as AxumResponse
};

use crate::{
    config::state::ApiState, 
    models::user::UserModel, 
    services::user::{find_user, oid_from_str},
    responses::{ApiResponse as ApiError, Response}, 
};

pub async fn is_valid_id(State(state): ApiState, Path(id): Path<String>, 
    mut req: Request, next: Next) -> Result<AxumResponse, ApiError> {

    let oid = oid_from_str(&id)?;
    let user = find_user(&state.db, doc! { "_id": oid }).await;

    if let Err(_) = user {
        return Err(Response::RESOURCE_NOT_FOUND)
    }

    req.extensions_mut().insert(oid);
    Ok(next.run(req).await)
}

pub async fn is_valid_id_and_token(State(state): ApiState,
    Path((id, token)): Path<(String, String)>, 
    mut req: Request, next: Next) -> Result<AxumResponse, ApiError> {

    let oid = oid_from_str(&id)?;
    let user = find_user(&state.db, doc! {"_id": oid}).await;

    if let Err(_) = user {
        return Err(Response::RESOURCE_NOT_FOUND)
    }

    req.extensions_mut().insert(oid);
    req.extensions_mut().insert(token);

    Ok(next.run(req).await)
}

pub async fn owner_validation(req: Request, 
    next: Next) -> Result<AxumResponse, ApiError> {
    
    let oid = req.extensions().get::<ObjectId>().unwrap().clone();
    let user = req.extensions().get::<UserModel>().unwrap().clone();

    if user.id != oid {
        return Err(Response::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}
