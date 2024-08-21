use crate::darkscout::types::member::MemberRole;
use crate::darkscout::types::workspace::MemberWorkspace;
use crate::darkscout::utils::auth::encrypt_password;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Encode};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct UserSignupForm {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub full_name: Option<String>,
    pub password: String,
    pub email: String,
    pub ip: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PasswordResetForm {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserEditForm {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub profile_picture: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct UserSignupResponseData {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
}
#[derive(Serialize, Deserialize)]
pub struct UserSignupResponse {
    pub data: Option<UserSignupResponseData>,
    pub err: Option<&'static str>,
}

#[derive(Serialize, Deserialize)]
pub struct VerifyUserForm {
    pub id: Uuid,
    pub code: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MemberProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub role: MemberRole,
    pub verified_domains: Option<Vec<String>>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub workspaces: Vec<MemberWorkspace>,
    pub auth: Auth,
    pub member_profile: Option<MemberProfile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    pub auth_token: String,
    pub refresh_token: String,
    pub access_token: String,
}

#[derive(FromRow, Encode, Clone, Debug)]
pub struct UserRecord {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: String,
    pub email: String,
    pub ip_str: String,
    pub password: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserData {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: String,
    pub email: String,
}
impl TryFrom<UserSignupForm> for UserRecord {
    type Error = &'static str;

    fn try_from(value: UserSignupForm) -> Result<Self, Self::Error> {
        let hashed = match encrypt_password(value.password) {
            Ok(hashed) => hashed,
            Err(_) => return Err("Failed to decrypt password"),
        };
        let mut first_name: Option<String> =
            Some(value.first_name.unwrap_or_else(|| "".to_string()));
        let mut last_name: Option<String> = Some(value.last_name.unwrap_or_else(|| "".to_string()));
        if let Some(full_name) = value.full_name {
            let mut sp = full_name.split(' ');
            first_name = Some(sp.next().unwrap_or("").to_string());
            last_name = Some(sp.next().unwrap_or("").to_string());
        }
        return Ok(UserRecord {
            id: Uuid::new_v4(),
            first_name,
            last_name,
            username: value.email.clone(),
            email: value.email,
            ip_str: value.ip.unwrap_or_else(|| "NA".to_string()),
            password: Some(hashed),
            created_at: None,
            updated_at: None,
        });
    }
}
