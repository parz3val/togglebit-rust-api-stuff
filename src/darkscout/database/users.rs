use crate::darkscout::types::store::UserRepo;
use crate::darkscout::types::{store::UserStore, user::UserRecord};
use sqlx::Error;
use uuid::Uuid;
use sqlx::{prelude::FromRow, Encode};

impl UserStore for UserRepo {
    async fn create_user(&self, user: &UserRecord) -> Result<Uuid, Error> {
        #[derive(FromRow)]struct MyID{id: Uuid}
        return match sqlx::query_as::<_, MyID>(
            r#"INSERT INTO USERS (ID, FIRST_NAME, LAST_NAME, USERNAME, EMAIL, IP_STR, PASSWORD) VALUES ($1, $2, $3, $4, $5, $6, $7) returning id"#).
            bind(&user.id).
            bind(&user.first_name).
            bind(&user.last_name).
            bind(&user.username).
            bind(&user.email).
            bind(&user.ip_str).
            bind(&user.password)
            .fetch_one(&self.db)
            .await {
            Ok(id) => Ok(id.id),
            Err(e) => {
                tracing::debug!("DBErr: {}", e);
                Err(e)
            }
            ,
        };
    }

    async fn get_by_email(&self, email: String) -> Result<UserRecord, Error> {
        return match sqlx::query_as::<_, UserRecord>(r#"SELECT * FROM USERS WHERE EMAIL = $1"#)
            .bind(email)
            .fetch_one(&self.db)
            .await
        {
            Ok(row) => Ok(row),
            Err(err) => Err(err),
        };
    }

    async fn get_by_id(&self, id: Uuid) -> Result<UserRecord, Error> {
        return match sqlx::query_as::<_, UserRecord>(r#""#)
            .bind(id)
            .fetch_one(&self.db)
            .await
        {
            Ok(row) => Ok(row),
            Err(e) => Err(e),
        };
    }

    async fn verify_user(&self, id: Uuid) -> Result<UserRecord, Error> {
        return match sqlx::query_as::<_, UserRecord>(
            r#"UPDATE USERS SET STATUS = 'VERIFIED' WHERE ID=$1 RETURNING *"#,
        )
        .bind(id)
        .fetch_one(&self.db)
        .await
        {
            Ok(user) => Ok(user),
            Err(e) => Err(e),
        };
    }

    async fn reset_user_password(
        &self,
        new_password_hash: String,
        user_id: Uuid,
    ) -> Result<(), Error> {
        return match sqlx::query(
            r#"
        UPDATE USERS SET PASSWORD = $1 WHERE ID = $2"#,
        )
        .bind(new_password_hash)
        .bind(user_id)
        .execute(&self.db)
        .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };
    }
    async fn update_user(&self, record: UserRecord) -> Result<(), Error> {
        return match sqlx::query(
            r#"
        UPDATE USERS SET
        FIRST_NAME = $1,
        LAST_NAME = $2,
        USERNAME = $3
        WHERE ID = $4
        "#,
        )
        .bind(record.first_name)
        .bind(record.last_name)
        .bind(record.username)
        .bind(record.id)
        .execute(&self.db)
        .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };
    }
}
