
use serde::{Deserialize, Serialize};

use mongodb::{
    Collection, Database,
    bson::{doc, oid::ObjectId, to_bson},
};

use crate::responses::{
    Response,
    ApiResult,
};

/// This struct store the complete user data
/// this is NOT shared with the client but is used for the database
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub validated: bool,
    pub profilePicture: String,
}

/// This struct store the public user data
/// that can be shared with other users and with the client
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserProfile {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub username: String,
    pub email: String,
    pub profilePicture: String,
}

impl UserModel {

    /// This method is an implementation of save functionality for the UserModel
    /// it saves the user data to the database. Similar to the *schema.save()* mongoose method
    pub async fn save(&self, db: &Database) -> ApiResult<()> {

        let users: Collection<UserModel> = db.collection("users");
        
        let serialized = to_bson(self)
            .map_err(|_| Response::INTERNAL_SERVER_ERROR)?
        ;
    
        let filter = doc! { "_id": self.id };
        let update = doc! { "$set": serialized };

        let _ = users.update_one(filter, update, None).await
            .map_err(|_| Response::INTERNAL_SERVER_ERROR)?
        ;

        Ok(())
    }
}


