
use bson::{doc, oid::ObjectId};
use axum::{extract::State, Extension};

use crate::{
    config::state::*, 
    services::user::find_user,
    models::{user::UserProfile, ToJson}, 
    responses::{ApiResponse, HttpResponse}, 
};

pub async fn get_user(State(state): ApiState, 
    Extension(oid): Extension<ObjectId>) -> HttpResponse {

    let filter = doc! {"_id": oid};
    let user = find_user(&state.db, filter).await?;

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
