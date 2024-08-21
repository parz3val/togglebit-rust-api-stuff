use crate::darkscout::types::store::{VerifiedEmailsRepo, VerifiedEmailsStore};
use crate::darkscout::types::verified_emails::VerifiedEmail;

impl VerifiedEmailsStore for VerifiedEmailsRepo {
    async fn add_verified_emails(
        &self,
        records: Vec<VerifiedEmail>,
    ) -> Result<(), sqlx::error::Error> {
        let mut transaction = self.db.begin().await?;
        let insert_query = "INSERT INTO VERIFIED_EMAILS (ID, EMAIL, ADDED_BY) VALUES ($1, $2, $3)";
        for record in records {
            sqlx::query(insert_query)
                .bind(&record.id)
                .bind(&record.email)
                .bind(&record.added_by)
                .execute(&mut *transaction)
                .await?;
            }
        transaction.commit().await?;
        Ok(())
    }

    async fn update_verified_email(&self, email: &str, is_verified: bool) -> Result<(), sqlx::error::Error> {
        match sqlx::query(
            r#"
        UPDATE VERIFIED_EMAILS SET
        IS_VERIFIED = $1,
        UPDATED_AT = CURRENT_TIMESTAMP
        WHERE EMAIL = $2
        "#,
        )
        .bind(is_verified)
        .bind(email)
        .execute(&self.db)
        .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
