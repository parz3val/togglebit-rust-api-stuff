use serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow, Type};
use uuid::Uuid;

use crate::darkscout::utils::auth::generate_code;

#[derive(Serialize, Deserialize, Clone, Type)]
#[sqlx(type_name = "INVITATION_STATUS")]
pub enum InvitationStatus {
    QUEUED,
    INVITED,
    FAILED,
    REJECTED,
    DELETED,
}

impl TryInto<String> for InvitationStatus {
    type Error = &'static str;
    fn try_into(self) -> Result<String, Self::Error> {
        return match self {
            InvitationStatus::QUEUED => Ok(String::from("QUEUED")),
            InvitationStatus::INVITED => Ok(String::from("INVITED")),
            InvitationStatus::FAILED => Ok(String::from("FAILED")),
            InvitationStatus::REJECTED => Ok(String::from("REJECTED ")),
            InvitationStatus::DELETED => Ok(String::from("DELETED")),
        };
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Type)]
#[sqlx(type_name = "TRANSPORT_TYPE")]
pub enum TransportType {
    EMAIL_CODE,
    PHONE_CODE,
    EMAIL_LINK,
    PHONE_LINK,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Type)]
#[sqlx(type_name = "INVITATION_TYPE")]
pub enum InvitationType {
    VERIFY_TOKEN,
    MEMBER_INVITATION,
}

impl TryInto<String> for InvitationType {
    type Error = & 'static str;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            InvitationType::MEMBER_INVITATION => Ok(String::from("MEMBER_INVITATION")),
            InvitationType::VERIFY_TOKEN => Ok(String::from("VERIFY_TOKEN")),
        }
    }
}

impl TryInto<String> for TransportType {
    type Error = &'static str;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            TransportType::EMAIL_CODE => Ok(String::from("EMAIL_CODE")),
            TransportType::PHONE_CODE => Ok(String::from("PHONE_CODE")),
            TransportType::EMAIL_LINK => Ok(String::from("EMAIL_LINK")),
            TransportType::PHONE_LINK => Ok(String::from("PHONE_LINK")),
        }
    }
}

#[derive(FromRow, Clone)]
pub struct Invitation {
    pub id: Uuid,
    pub subject: String,
    pub msg: String,
    pub email: String,
    pub details: String,
    pub created_by: Uuid,
    pub sent_to: Uuid,
    pub workspace: Uuid,
    pub transport: TransportType,
    pub status: InvitationStatus,
    pub inv_type: InvitationType,
}

#[derive(FromRow, Clone, Decode)]
pub struct InvitationRecord {
    pub id: Uuid,
    pub subject: String,
    pub msg: String,
    pub email: String,
    pub details: String,
    pub created_by: Uuid,
    pub sent_to: Uuid,
    pub workspace: Uuid,
    pub transport: String,
    pub status: String,
}

#[derive(FromRow, Clone, Decode)]
pub struct VerificationData {
    pub details: String,
    pub created_by: Uuid,
    pub sent_to: Uuid,
    pub workspace: Uuid,
    pub transport: TransportType,
    pub status: InvitationStatus,
}

pub fn new_verification_code(
    user: Uuid,
    inviter: Uuid,
    workspace: Uuid,
    email: String,
) -> Invitation {
    let code = generate_code();
    let msg = format!("<h1>Your verification code for darkscout account is {} </h1>", &code);
    Invitation {
        id: Uuid::new_v4(),
        subject: String::from("DarkScout: Verification Email"),
        details: code,
        msg,
        email,
        created_by: inviter,
        sent_to: user,
        workspace,
        transport: TransportType::EMAIL_CODE,
        status: InvitationStatus::QUEUED,
        inv_type: InvitationType::VERIFY_TOKEN,
    }
}


pub fn new_workspace_invitation(
    user_id: Uuid, name: Option<String>, workspace_name: String, email: String, workspace_id: Uuid, invited_user: Uuid
) -> Invitation {

    let name = name.unwrap_or_else(|| String::from("default"));
    let link = String::from("https://darkscout.com/invitations/user_id/");
    let email_link = format!("{} Link", link);
    Invitation{
        id: Uuid::new_v4(),
        subject: format!("Hello {}. You've been invited to {} on darkscout.", &name, workspace_name),
        msg: format!("Hello {}. You've been invited to darkscout by {}. Please visit {} to accept invitation", &name, &workspace_name, email_link),
        email,
        details: link,
        created_by: user_id,
        sent_to: invited_user,
        workspace: workspace_id,
        transport: TransportType::EMAIL_LINK,
        status: InvitationStatus::QUEUED,
        inv_type: InvitationType::MEMBER_INVITATION
    }

}
