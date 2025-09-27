use serde::Serialize;
use chrono::NaiveDate;

#[derive(Serialize)]
pub struct UserResponse {
    pub userid: i32,
    pub fullname: String,
    pub birthday: NaiveDate,
    pub sex: String,
    pub address: Option<String>,
    pub contact: Option<String>,
    pub email: String,
    pub is_banned: bool,
}

impl From<crate::features::user::model::Model> for UserResponse {
    fn from(user: crate::features::user::model::Model) -> Self {
        Self {
            userid: user.userid,
            fullname: user.fullname,
            birthday: user.birthday,
            sex: user.sex,
            address: user.address,
            contact: user.contact,
            email: user.email,
            is_banned: user.is_banned,
        }
    }
}
