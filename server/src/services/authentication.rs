
use mongodb::{
    Collection,
    Database,
};

use bson::{doc, oid::ObjectId};

use crate::{
    
    responses::{
        Response,
        ApiResult,
    },
    
    models::authentication::Token,
};

pub async fn save_token(token_type: &str, db: &Database, 
    token: &String, user_id: ObjectId) -> ApiResult<()> {

    let tokens: Collection<Token> = db.collection(token_type);

    let token = Token {
        id: ObjectId::new(),
        token: token.clone(),
        user_id
    };

    let _ = tokens.insert_one(&token, None).await
        .map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
    ;

    Ok(())
}

pub async fn is_token(token_type: &str, db: &Database, token: &String) -> ApiResult<bool> {

    let tokens: Collection<Token> = db.collection(token_type);

    let query = tokens.find_one(doc! { "token": token }, None)
        .await.map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
    ;

    match query {
        Some(_) => return Err(Response::UNAUTHORIZED),
        None => return Ok(false)
    }
}
