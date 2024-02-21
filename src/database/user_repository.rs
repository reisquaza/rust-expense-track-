use sqlx::{AnyPool, Row, Error};
use crate::entity::user_entity::{UserDTO, UserEntity}; 
use uuid::Uuid;

#[derive(Debug)]
struct UserRepository<'any_pool> {
    _pool: &'any_pool AnyPool,
}

impl <'any_pool> UserRepository<'any_pool> {
    fn new(pool: &'any_pool AnyPool) -> Self {
        UserRepository { _pool: pool }
    }

  pub async fn create_user(&self, user_dto: UserDTO) -> Result<UserEntity, Error> {
        let mut conn = self._pool.acquire().await?;
        let query = "INSERT INTO users (id, name, email, password) VALUES ($1, $2, $3, $4)";
        let id = Uuid::new_v4();
        let row = sqlx::query(query)
            .bind(id.to_string())
            .bind(&user_dto.name)
            .bind(&user_dto.email)
            .bind(&user_dto.password)
            .fetch_one(&mut *conn)
            .await?;
        Ok(UserEntity {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            password: row.get("password")
        })
    }

    pub async fn get_user(&self, id: String) -> Result<Option<UserEntity>, Error> {
        let mut conn = self._pool.acquire().await?;
        let query = "SELECT * FROM users WHERE id = $1";
        let row = sqlx::query(query)
            .bind(id)
            .fetch_optional(&mut *conn)
            .await?;
        let user = if let Some(row) = row {
            Some(UserEntity {
                id: row.get("id"),
                name: row.get("name"),
                email: row.get("email"),
                password: row.get("password")
            })
        } else {
            None 
        };
        Ok(user)
    }

}
