use crate::darkscout::types::workspace::{MemberWorkspace, MemberWorkspaceDetails};
use crate::darkscout::types::{
    store::{WorkspaceStore},
    workspace::Workspace,
};
use sqlx::Error;
use uuid::Uuid;
use crate::darkscout::types::store::WorkspaceRepo;

impl WorkspaceStore for WorkspaceRepo {
    async fn create_workspace(&self, record: Workspace) -> Result<Workspace, sqlx::error::Error> {
        return match sqlx::query(
            r#"INSERT INTO WORKSPACES (ID, TITLE, CREATED_BY) VALUES ($1, $2, $3)"#,
        )
        .bind(&record.id)
        .bind(&record.title)
        .bind(&record.created_by)
        .execute(&self.db)
        .await
        {
            Ok(_) => Ok(record),
            Err(e) => Err(e),
        };
    }
    async fn create_member_workspace_details(
        &self,
        record: MemberWorkspaceDetails,
    ) -> Result<(), sqlx::error::Error> {
        return match sqlx::query(
            r#"
            insert into member_workspace_details
            (id, workspace_order, is_default, member_id) values
            ($1, $2, $3, $4)
            "#,
        )
        .bind(&record.id)
        .bind(&record.workspace_order)
        .bind(&record.is_default)
        .bind(&record.member_id)
        .execute(&self.db)
        .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };
    }

    async fn get_admin_workspaces(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Workspace>, sqlx::error::Error> {
        let workspaces: Vec<Workspace> =
            sqlx::query_as(r#"SELECT * FROM WORKSPACES WHERE CREATED_BY=$1"#)
                .bind(user_id)
                .fetch_all(&self.db)
                .await?;
        Ok(workspaces)
    }
    async fn get_user_workspaces(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<MemberWorkspace>, sqlx::error::Error> {
        let workspaces: Vec<MemberWorkspace> = sqlx::query_as(
            r#"
        WITH WORKSPACE_IDS AS (SELECT WORKSPACE FROM MEMBERS WHERE USER_ID = $1)
        SELECT *
        FROM WORKSPACES LEFT JOIN MEMBER_WORKSPACE_DETAILS
        ON WORKSPACES.ID = MEMBER_WORKSPACE_DETAILS.ID
        WHERE WORKSPACES.ID IN (SELECT WORKSPACE FROM WORKSPACE_IDS)
        "#,
        )
        .bind(user_id)
        .fetch_all(&self.db)
        .await?;
        Ok(workspaces)
    }

    async fn get_member_workspace(&self, member_id: Uuid) -> Result<Workspace, sqlx::error::Error> {
        return match sqlx::query_as::<_, Workspace>(
            r#"
        SELECT * FROM WORKSPACES
        LEFT JOIN MEMBER_WORKSPACE_DETAILS ON WORKSPACES.ID = MEMBER_WORKSPACE_DETAILS.ID
        WHERE WORKSPACES.ID = MEMBER_WORKSPACE_DETAILS.ID"#,
        )
        .bind(member_id)
        .fetch_one(&self.db)
        .await
        {
            Ok(w) => Ok(w),
            Err(e) => Err(e),
        };
    }
    async fn update_workspace(&self, record: Workspace) -> Result<(), sqlx::error::Error> {
        return match sqlx::query(
            r#"update workspaces set title = $1, details = $2 where id = $3"#,
        )
            .bind(&record.title)
            .bind(&record.details)
            .bind(&record.id)
            .execute(&self.db)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };
    }
}
