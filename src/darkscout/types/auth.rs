use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;
use crate::darkscout::types::user::{MemberProfile, UserData};
use crate::darkscout::types::workspace::MemberWorkspace;

#[derive(Serialize, Deserialize)]
pub struct LoggedInUserClaims {
    pub workspace: Option<MemberWorkspace>,
    pub member: Option<MemberProfile>,
    pub user: Option<UserData>,

}
#[derive(Serialize, Clone, Type, Deserialize)]
#[sqlx(type_name="NOTIFICATION_STATUS")]
pub enum NotificationStatus {
    CREATED,
    EXPIRED,
    DELETED,
    USED
}
#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct PasswordResetRequest {
    pub id: Uuid,
    pub token: String,
    pub expires_after: i32,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub update_at: Option<chrono::DateTime<Utc>>,
    pub status: NotificationStatus,
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RequestResetLinkForm {
    pub email: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResetPasswordWithToken {
    pub token: String,
    pub new_password: String,
    pub user_id: Uuid,
}
