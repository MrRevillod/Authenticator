
use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId}
};

use axum::{extract::State, Extension};

use crate::{
    
    config::state::*, 

    models::{
        ToJson,
        user::{UserModel, UserProfile}, 
    }, 
    responses::{
        ApiResponse, HttpResponse, Response
    }, 
};

pub async fn get_user(State(state): ApiState, 
    Extension(oid): Extension<ObjectId>) -> HttpResponse {

    let users: Collection<UserModel> = state.db.collection("users");
    let query = users.find_one(doc! {"_id": oid}, None)
        .await.map_err(|_| Response::INTERNAL_SERVER_ERROR)?
    ;

    let user = match query {
        Some(user) => user,
        None => return Err(Response::RESOURCE_NOT_FOUND)
    };

    let profile = UserProfile {
        id: user.id.to_hex(),
        name: user.name,
        username: user.username,
        email: user.email,
        profilePicture: user.profilePicture,
    };

    Ok(ApiResponse::DataResponse(
        200, "success", "user", profile.to_json()
    ))
}
