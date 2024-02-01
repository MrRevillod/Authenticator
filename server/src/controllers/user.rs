
use std::collections::HashMap;

use axum::{extract::State, Extension, Json};
use bson::{doc, oid::ObjectId, Document};
use mongodb::Collection;
use serde_json::{from_value, to_value, Value};

use crate::{
    config::state::*, models::user::{UserModel, UserProfile}, responses::{ApiResponse, HttpResponse, Response}, services::user::email_change_mailer 
};

pub async fn get_user(State(state): ApiState, 
    Extension(oid): Extension<ObjectId>) -> HttpResponse {

    let users: Collection<UserModel> = state.db.collection("users");

    let query = users.find_one(doc! {"_id": oid}, None)
        .await.map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
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
    };

    Ok(ApiResponse::DataResponse(
        200, "success", "user", to_value(profile).unwrap()
    ))
}

pub async fn delete_account(State(state): ApiState,
    Extension(oid): Extension<ObjectId>) -> HttpResponse {

    let users: Collection<UserModel> = state.db.collection("users");

    let query = users.delete_one(doc! {"_id": oid}, None)
        .await.map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
    ;

    if query.deleted_count == 0 {
        return Err(Response::RESOURCE_NOT_FOUND);
    }

    Ok(Response::SUCCESS)
}

pub async fn update_profile(State(state): ApiState, 
    Extension(oid): Extension<ObjectId>, Json(body): Json<Value>) -> HttpResponse {

    let users: Collection<UserModel> = state.db.collection("users");
    let valid_fields = vec!["id", "name", "username", "email"];

    let body_map: HashMap<String, String> = from_value(body)
        .map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
    ;

    let mut doc = Document::new();

    for (key, value) in &body_map {
        
        if !valid_fields.contains(&key.as_str()) {
            return Err(Response::BAD_REQUEST);
        }

        if key == "email" {

            let token = format!("{}{}", &JWT_SECRET.to_string(), &value);
            let url = format!("{}/users/change-email/{}/{}", 
                CLIENT_ADDR.to_string(), oid.to_hex(), token
            );

            email_change_mailer(key, &url).await?;
        }
        
        doc.insert(key, value);
    }

    let query = users.update_one(doc! {"_id": oid}, doc, None)
        .await.map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
    ;

    if query.modified_count == 0 {
        return Err(Response::RESOURCE_NOT_FOUND);
    }

    Ok(Response::SUCCESS)
}







