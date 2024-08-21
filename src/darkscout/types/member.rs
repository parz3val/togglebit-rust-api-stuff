use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

#[derive(Serialize, Clone, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "MEMBER_STATUS")]
pub enum MemberStatus {
    VERIFIED,
    SUSPENDED,
    DELETED,
}

#[derive(Serialize, Clone, Deserialize, Type, PartialEq, Debug)]
#[sqlx(type_name = "MEMBER_ROLE")]
pub enum MemberRole {
    ADMIN,
    OWNER,
    MEMBER,
    GUEST,
}

#[derive(Serialize, Clone, Deserialize, FromRow, sqlx::Encode)]
pub struct MemberRecord {
    pub id: Uuid,
    pub joined_at: DateTime<Utc>,
    pub profile_picture: Option<String>,
    pub status: Option<MemberStatus>,
    pub role: Option<MemberRole>,
    pub invitation_token: Option<String>,
    pub invitation_code: Option<i32>,
    pub user_id: Uuid,
    pub invited_by: Uuid,
    pub role_granted_by: Option<Uuid>,
    pub workspace: Uuid,
}

#[derive(Serialize, Deserialize, FromRow, sqlx::Encode, Clone)]
pub struct MemberData {
    pub id: Uuid,
    pub joined_at: NaiveDateTime,
    pub profile_picture: String,
    pub status: MemberStatus,
    pub role: MemberRole,
    pub user_id: Uuid,
    pub invited_by: Uuid,
    pub workspace: Uuid,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
