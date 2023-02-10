use serde::Deserialize;

#[derive(Deserialize)] 
pub struct NewUserSchema {
    pub username: String,
    pub email: String,
    pub password: String
}