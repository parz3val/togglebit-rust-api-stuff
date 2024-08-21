use sqlx::error::Error;
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

use crate::darkscout::types::member::MemberData;
use crate::darkscout::types::store::MemberRepo;
use crate::darkscout::types::{
    member::MemberRecord,
    store::{MemberStore, PgStore},
};

impl MemberStore for MemberRepo {
    async fn create_new_member(&self, record: MemberRecord) -> Result<(), Error> {
        return match sqlx::query(
            r#"
            INSERT INTO MEMBERS
            (
            ID, JOINED_AT, PROFILE_PICTURE, STATUS, ROLE, WORKSPACE,
            USER_ID, INVITED_BY,
            ROLE_GRANTED_BY
            )
            VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind(&record.id)
        .bind(&record.joined_at)
        .bind(&record.profile_picture)
        .bind(&record.status)
        .bind(&record.role)
        .bind(&record.workspace)
        .bind(&record.user_id)
        .bind(&record.invited_by)
        .bind(&record.role_granted_by)
        .execute(&self.db)
        .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };
    }

    async fn get_member_profile(&self, user_id: Uuid) -> Result<MemberData, Error> {
        let member: MemberData = sqlx::query_as(r#"SELECT * FROM MEMBERS WHERE USER_ID=$1"#)
            .bind(user_id)
            .fetch_one(&self.db)
            .await?;
        Ok(member)
    }

    async fn update_member_profile_picture(
        &self,
        member_id: Uuid,
        url: String,
    ) -> Result<(), Error> {
        match sqlx::query(r#"UPDATE MEMBERS SET PROFILE_PICTURE = $2 WHERE ID = $1"#)
            .bind(member_id)
            .bind(url)
            .execute(&self.db)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
    async fn get_member_settings(&self, member_id: Uuid) -> Result<(), Error> {
        Ok(())
    }
}
