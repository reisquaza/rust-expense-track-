#[derive(Debug)]
pub struct UserEntity {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}

pub struct UserDTO {
    pub name: String,
    pub email: String,
    pub password: String
}