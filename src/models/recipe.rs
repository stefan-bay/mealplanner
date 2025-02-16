pub use super::_entities::recipe::{ActiveModel, Model, Entity, Column};
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait, InsertResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AddBody {
    pub title: String,
    pub instructions: String,
}


// implement your read-oriented logic here
impl Model {
    pub async fn create(db: &DatabaseConnection, body: &AddBody) -> Result<InsertResult<ActiveModel>, DbErr> {
        let new = ActiveModel {
            id: ActiveValue::NotSet,
            title: ActiveValue::Set(body.title.to_owned()),
            instructions: ActiveValue::Set(body.instructions.to_owned()),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            updated_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        };
        let insert_result = Entity::insert(new).exec(db).await;

        match insert_result {
            Ok(_) => insert_result,
            Err(err) => {
                println!("insert result: {:?}", err);
                Err(err)
            }
        }
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
