
use futures::stream::TryStreamExt;
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};
use bson::{doc, oid::ObjectId, to_bson};

use crate::responses::{
    Response,
    ApiResult,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub validated: bool,
    pub tasks: Vec<ObjectId>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserProfile {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub username: String,
    pub email: String,
}

impl UserModel {

    pub async fn save(&self, db: &Database) -> ApiResult<()> {

        let users: Collection<UserModel> = db.collection("users");
        
        let serialized = to_bson(self)
            .map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
        ;
    
        let filter = doc! { "_id": self.id };
        let update = doc! { "$set": serialized };

        let _ = users.update_one(filter, update, None).await
            .map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
        ;

        Ok(())
    }
    
    #[allow(dead_code)]
    pub async fn find_all(db: &Database) -> ApiResult<Vec<UserModel>> {
        
        let collection: Collection<UserModel> = db.collection("user");

        let mut cursor = collection.find(None, None).await
            .map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
        ;

        let mut users: Vec<UserModel> = Vec::new();

        while let Some(user) = cursor.try_next().await
            .map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
        {
            users.push(user);
        }

        Ok(users)
    }
}


