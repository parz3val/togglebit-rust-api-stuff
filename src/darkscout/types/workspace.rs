use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::user::UserRecord;

#[derive(Serialize, Deserialize, sqlx::FromRow, sqlx::Encode, sqlx::Decode, Debug, Clone)]
pub struct Workspace {
    pub id: Uuid,
    pub title: Option<String>,
    pub details: Option<String>,
    pub created_by: Uuid,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, sqlx::Encode, sqlx::Decode, Debug, Clone)]
pub struct WorkspaceSettings{
    pub id: Uuid,
    pub member_id: Uuid,
    pub workspace_id: Uuid,
    pub settings_name: String,
    pub settings_value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct WorkspaceSettingsForm<'a> {
    pub settings_name: & 'a str,
    pub settings_value: & 'a str,
}

#[derive(Serialize, Deserialize)]
pub struct EditWorkspaceForm {
    pub id: Uuid,
    pub title: Option<String>,
    pub details: Option<String>,
    pub order: Option<i32>,
    pub display_picture: Option<String>,
    pub is_default: Option<bool>,
    pub is_fav: Option<bool>
}

#[derive(Serialize, Deserialize, sqlx::FromRow, sqlx::Encode,sqlx::Decode )]
pub struct WorkspaceForm {
    pub title: Option<String>,
    pub details: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct InvitationForm {
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct MemberInvitationsForm {
    pub(crate) invitations: Vec<InvitationForm>
}

#[derive(Serialize, Deserialize)]
pub enum InvitationResultStatus {
    SUCCESS,
    FAILURE,
} 


#[derive(Serialize, Deserialize)]
pub struct MemberInvitationResponseData {
    pub email: String,
    pub status: InvitationResultStatus,
}

#[derive(Serialize, Deserialize)]
pub struct AcceptMemberInvitationForm {
    pub user_id: Uuid,
    pub token: String,
}
    
#[derive(Serialize, Deserialize, sqlx::FromRow, sqlx::Encode,sqlx::Decode, Clone, Debug )]
pub struct MemberWorkspace {
    pub id: Uuid,
    pub title: Option<String>,
    pub details: Option<String>,
    pub created_by: Uuid,
    pub workspace_order: i32,
    pub is_default: bool,
    pub member_id: Uuid,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, sqlx::Encode,sqlx::Decode )]
pub struct MemberWorkspaceDetails {
    pub id: Uuid,
    pub workspace_order: i32,
    pub is_default: bool,
    pub is_fav: bool,
    pub member_id: Uuid
}
impl TryFrom<UserRecord> for Workspace {
    type Error = &'static str;

    fn try_from(value: UserRecord) -> Result<Self, Self::Error> {
        return Ok(Workspace {
            id: Uuid::new_v4(),
            title: Some(String::from("default")),
            details: Some(format!("Default workspace for {:}", value.email)),
            created_by: value.id,
        });
    }
}
