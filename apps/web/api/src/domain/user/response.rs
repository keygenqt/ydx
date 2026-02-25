use serde::Serialize;

use crate::{api::api_response::ApiResponse, domain::user::model::UserModel};

/// Model response [UserModel]
#[derive(Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub email: String,
    pub icon: Option<String>,
    pub fname: Option<String>,
    pub lname: Option<String>,
}

/// Impl response
impl UserResponse {
    pub fn from_model(model: &UserModel) -> Result<UserResponse, ApiResponse> {
        if model.id.is_none() {
            Err(ApiResponse::new_500())?;
        }
        Ok(Self {
            id: model.id.unwrap(),
            email: model.email.clone(),
            icon: model.icon.clone(),
            fname: model.fname.clone(),
            lname: model.lname.clone(),
        })
    }
}
