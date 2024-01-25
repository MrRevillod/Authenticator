
use mongodb::{
    Collection, 
    Database, 
    bson::Document
};

use crate::{
    
    models::user::UserModel,
    
    responses::{
        ApiResult,
        Response
    },
};

pub async fn check_user_exists(
    db: &Database, filter: Document) -> ApiResult<bool> {

    let users: Collection<UserModel> = db.collection("users");

    let query = users.find_one(filter, None).await
        .map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
    ;

    match query {
        Some(_) => Err(Response::USER_ALREADY_EXISTS),
        None => return Ok(false)
    }
}