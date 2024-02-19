
use std::{collections::HashMap, str::FromStr};

use bson::{oid::ObjectId, Document};
use mongodb::{
    Collection, 
    Database, 
    bson::doc
};

use crate::{
    models::{user::UserModel, ToJson},
    responses::{ApiResponse, ApiResult, Response},
};

pub fn oid_from_str(oid: &String) -> ApiResult<ObjectId> {

    match ObjectId::from_str(oid) {
        Ok(oid) => Ok(oid),
        Err(_) => Err(Response::BAD_REQUEST)
    }
}

pub async fn find_user(db: &Database, filter: Document) -> ApiResult<UserModel> {

    let users: Collection<UserModel> = db.collection("users");
    let user = users.find_one(filter, None).await
        .map_err(|_| Response::INTERNAL_SERVER_ERROR)?
    ;

    match user {
        Some(user) => Ok(user),
        None => Err(Response::RESOURCE_NOT_FOUND)
    }
}

pub async fn check_conflict_fields(db: &Database, body_map: &HashMap<String, String>) -> ApiResult<()> {

    let users: Collection<UserModel> = db.collection("users");
    let mut existing_fields = HashMap::new();

    if body_map.get("email").is_some() {
        
        let email_exists = users.find_one(doc! {"email": &body_map["email"]}, None).await
            .map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
            .is_some()
        ;
        
        if email_exists {
            existing_fields.insert("email", "Ya existe un usuario con este email");
        }
    }

    if body_map.get("username").is_some() {
        
        let username_exists = users.find_one(doc! {"username": &body_map["username"]}, None).await
            .map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
            .is_some()
        ;
        
        if username_exists {
            existing_fields.insert("username", "Ya existe un usuario con este alias");
        }
    }

    if !existing_fields.is_empty() {
        return Err(ApiResponse::DataResponse(
            409, "Conflicto", "conflicts", existing_fields.to_json())
        )
    }

    Ok(())
}
