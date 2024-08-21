use crate::darkscout::types::invitations::{InvitationRecord, InvitationStatus, VerificationData};
use crate::darkscout::types::{
    store::{InvitationsStore},
};
use chrono::{Local};
use tracing::debug;
use uuid::Uuid;
use crate::darkscout::types::invitations::Invitation;
use crate::darkscout::types::store::InvitationRepo;

impl InvitationsStore for InvitationRepo {
    async fn create_new(&self, record: Invitation) -> Result<Invitation, sqlx::error::Error> {
        let record_ = record.clone();
        return match sqlx::query(r#"insert into invitations
        (id, subject, msg, email, details, created_by, sent_to, workspace) values ($1, $2, $3, $4, $5, $6, $7, $8)"#)
            .bind(record.id)
            .bind(record.subject)
            .bind(record.msg)
            .bind(record.email)
            .bind(record.details)
            .bind(record.created_by)
            .bind(record.sent_to)
            .bind(record.workspace)
            .execute(&self.db).await {
            Ok(_) => Ok(record_),
            Err(err) => {
                tracing::debug!("DBErr: Failed to insert into invitations {}", err);
                Err(err)
            },
        };
    }

    async fn get_active_by_id(&self, id: Uuid) -> Result<InvitationRecord, sqlx::error::Error> {
        return match sqlx::query_as::<_, InvitationRecord>(
            r#"SELECT * FROM INVITATIONS WHERE ID = $1 AND STATUS != 'DELETED'"#,
        )
        .bind(id)
        .fetch_one(&self.db)
        .await
        {
            Ok(inv) => Ok(inv),
            Err(e) => Err(e),
        };
    }
    async fn get_verification_data(
        &self,
        id: &Uuid,
    ) -> Result<VerificationData, sqlx::error::Error> {
        return match sqlx::query_as::<_, VerificationData>(
            r#"SELECT * FROM INVITATIONS WHERE SENT_TO = $1 AND STATUS != 'DELETED'"#,
        )
        .bind(id)
        .fetch_one(&self.db)
        .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        };
    }
    async fn update_invitation_status(
        &self,
        status: InvitationStatus,
        id: Uuid,
    ) -> Result<(), sqlx::error::Error> {
        let ts = Local::now().naive_utc();
        return match sqlx::query(
            r#"UPDATE INVITATIONS SET STATUS = $1, UPDATED_AT = $2 WHERE CREATED_BY = $3"#,
        )
        .bind(status as InvitationStatus)
        .bind(ts)
        .bind(id)
        .execute(&self.db)
        .await
        {
            Ok(result) => {
                dbg!(result);
                Ok(())
            }
            Err(e) => Err(e),
        };
    }
}
