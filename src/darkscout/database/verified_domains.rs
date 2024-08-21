use crate::darkscout::types::store::{VerifiedDomainRepo, VerifiedDomainsStore};
use crate::darkscout::types::verified_domains::VerifiedDomain;

impl VerifiedDomainsStore for VerifiedDomainRepo {
    async fn add_verified_domains(
        &self,
        records: Vec<VerifiedDomain>,
    ) -> Result<(), sqlx::error::Error> {
        let mut transaction = self.db.begin().await?;
        let insert_query = "INSERT INTO VERIFIED_DOMAINS (ID, DOMAIN, ADDED_BY) VALUES ($1, $2, $3)";
        for record in records {
            sqlx::query(insert_query)
                .bind(&record.id)
                .bind(&record.domain)
                .bind(&record.added_by)
                .execute(&mut *transaction)
                .await?;
            }
        transaction.commit().await?;
        Ok(())
    }

    async fn update_verified_domain(&self, domain: &str, is_verified: bool) -> Result<(), sqlx::error::Error> {
        match sqlx::query(
            r#"
        UPDATE VERIFIED_DOMAINS SET
        IS_VERIFIED = $1,
        UPDATED_AT = CURRENT_TIMESTAMP
        WHERE DOMAIN = $2
        "#,
        )
        .bind(is_verified)
        .bind(domain)
        .execute(&self.db)
        .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

}
