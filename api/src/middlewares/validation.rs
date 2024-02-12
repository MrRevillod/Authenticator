
use std::str::FromStr;

use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId}, 
};

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
    responses::{ApiResponse as ApiError, Response}
};

pub async fn is_valid_id(State(state): ApiState, 
    Path(id): Path<String>, mut req: Request, next: Next) -> Result<AxumResponse, ApiError> {

    let users: Collection<UserModel> = state.db.collection("users");

    let oid = match ObjectId::from_str(&id) {
        Ok(oid) => oid,
        Err(_) => return Err(Response::BAD_REQUEST)
    };

    let query = users.find_one(doc! {"_id": oid}, None)
        .await.map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
    ;

    if let None = query {
        return Err(Response::RESOURCE_NOT_FOUND)
    }

    req.extensions_mut().insert(oid);

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

