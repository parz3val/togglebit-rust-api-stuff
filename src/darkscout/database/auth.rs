use crate::darkscout::types::auth::PasswordResetRequest;
use crate::darkscout::types::store::{AuthRepo, AuthStore, PgStore};
use sqlx::postgres::PgQueryResult;
use sqlx::Error;
use uuid::Uuid;

impl AuthStore for AuthRepo {
    async fn create_password_reset_request(
        &self,
        record: PasswordResetRequest,
    ) -> Result<(), Error> {
        match sqlx::query(
            r#"
        INSERT INTO PASSWORD_RESET_REQUESTS
        ( ID, TOKEN, EXPIRES_AFTER, USER_ID) VALUES
        ($1, $2, $3, $4)
        "#,
        )
        .bind(record.id)
        .bind(record.token)
        .bind(record.expires_after)
        .bind(record.user_id)
        .execute(&self.db)
        .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    async fn get_reset_request(
        &self,
        token: String,
        user_id: Uuid,
    ) -> Result<PasswordResetRequest, Error> {
        return match sqlx::query_as::<_, PasswordResetRequest>(
            r#"selet * from password_reset_requests where token = $1 and user_id = $2"#,
        )
        .bind(token)
        .bind(user_id)
        .fetch_one(&self.db)
        .await
        {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        };
    }
}
