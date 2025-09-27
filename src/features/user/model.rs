use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub userid: i32,
    pub fullname: String,
    pub birthday: NaiveDate, 
    pub sex: String,
    pub address: Option<String>,
    pub contact: Option<String>,
    pub email: String,
    pub password: String,
    pub is_banned: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
