use crate::darkscout::types::auth::PasswordResetRequest;
use crate::darkscout::types::invitations::{InvitationRecord, InvitationStatus};
use crate::darkscout::types::member::MemberData;
use crate::darkscout::types::verified_emails::VerifiedEmail;
use crate::darkscout::types::workspace::{MemberWorkspace, MemberWorkspaceDetails};
use crate::darkscout::types::verified_domains::VerifiedDomain;
use mongodb::Client;
use sqlx::{error::Error, Pool, Postgres};
use std::sync::Arc;
use moka::future;
use uuid::Uuid;

use super::{
    invitations::{Invitation, VerificationData},
    member::MemberRecord,
    user::UserRecord,
    workspace::Workspace,
    Settings,
};

pub trait NewDb {
    fn new(settings: &Settings) -> impl std::future::Future<Output = Self>
    where
        Self: Sized;
}
#[derive(Clone)]
pub struct PgStore {
    pub db: Pool<Postgres>,
    pub user: Arc<UserRepo>,
    pub workspace: Arc<WorkspaceRepo>,
    pub invitation: Arc<InvitationRepo>,
    pub member: Arc<MemberRepo>,
    pub verified_emails: Arc<VerifiedEmailsRepo>,
    pub verified_domains: Arc<VerifiedDomainRepo>,
    pub auth: Arc<AuthRepo>,
}

#[derive(Clone)]
pub struct DSCache {
    pub l3: Client,
    pub l1: future::Cache<String, String>,
}

#[derive(Clone)]
pub struct UserRepo {
    pub db: Pool<Postgres>,
}
#[derive(Clone)]
pub struct WorkspaceRepo {
    pub db: Pool<Postgres>,
}
#[derive(Clone)]
pub struct InvitationRepo {
    pub db: Pool<Postgres>,
}
#[derive(Clone)]
pub struct MemberRepo {
    pub db: Pool<Postgres>,
}

#[derive(Clone)]
pub struct VerifiedEmailsRepo {
    pub db: Pool<Postgres>,
}

#[derive(Clone)]
pub struct VerifiedDomainRepo {
    pub db: Pool<Postgres>,
}

#[derive(Clone)]
pub struct AuthRepo {
    pub db: Pool<Postgres>,
}
#[allow(async_fn_in_trait)]
pub trait UserStore {
    async fn create_user(&self, user: &UserRecord) -> Result<Uuid, Error>;
    async fn get_by_email(&self, email: String) -> Result<UserRecord, Error>;
    async fn get_by_id(&self, id: Uuid) -> Result<UserRecord, sqlx::error::Error>;
    async fn verify_user(&self, id: uuid::Uuid) -> Result<UserRecord, Error>;
    async fn reset_user_password(
        &self,
        new_password_hash: String,
        user_id: Uuid,
    ) -> Result<(), Error>;
    async fn update_user(&self, record: UserRecord) -> Result<(), Error>;
}

#[allow(async_fn_in_trait)]
pub trait WorkspaceStore {
    async fn create_workspace(&self, record: Workspace) -> Result<Workspace, Error>;
    async fn create_member_workspace_details(
        &self,
        record: MemberWorkspaceDetails,
    ) -> Result<(), sqlx::error::Error>;
    async fn get_admin_workspaces(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Workspace>, sqlx::error::Error>;
    async fn get_user_workspaces(&self, user_id: Uuid) -> Result<Vec<MemberWorkspace>, Error>;
    async fn get_member_workspace(&self, member_id: Uuid) -> Result<Workspace, sqlx::error::Error>;
    async fn update_workspace(&self, record: Workspace) -> Result<(), sqlx::error::Error>;
}

#[allow(async_fn_in_trait)]
pub trait InvitationsStore {
    async fn create_new(&self, record: Invitation) -> Result<Invitation, Error>;
    async fn get_active_by_id(&self, id: Uuid) -> Result<InvitationRecord, Error>;
    async fn get_verification_data(&self, id: &Uuid) -> Result<VerificationData, Error>;
    async fn update_invitation_status(
        &self,
        status: InvitationStatus,
        id: Uuid,
    ) -> Result<(), Error>;
}

#[allow(async_fn_in_trait)]
pub trait MemberStore {
    async fn create_new_member(&self, record: MemberRecord) -> Result<(), Error>;
    async fn get_member_profile(&self, user_id: Uuid) -> Result<MemberData, Error>;
    async fn update_member_profile_picture(
        &self,
        member_id: Uuid,
        url: String,
    ) -> Result<(), Error>;
    async fn get_member_settings(&self, member_id: Uuid) -> Result<(), Error>;
}


#[allow(async_fn_in_trait)]
pub trait VerifiedEmailsStore {
    async fn add_verified_emails(&self, records: Vec<VerifiedEmail>)->Result<(), Error>;
    async fn update_verified_email(&self, email: &str, is_verified: bool)->Result<(), Error>;
}

#[allow(async_fn_in_trait)]
pub trait VerifiedDomainsStore {
    async fn add_verified_domains(&self, records: Vec<VerifiedDomain>)->Result<(), Error>;
    async fn update_verified_domain(&self, domain: &str, is_verified: bool)->Result<(), Error>;
}

#[allow(async_fn_in_trait)]
pub trait AuthStore {
    async fn create_password_reset_request(
        &self,
        record: PasswordResetRequest,
    ) -> Result<(), Error>;
    async fn get_reset_request(
        &self,
        token: String,
        user_id: Uuid,
    ) -> Result<PasswordResetRequest, Error>;
}

#[allow(async_fn_in_trait)]
pub trait CacheStore {
    async fn get(&self, key: &str) -> Result<(), Error>;
    // async fn set(&self, key: &str, value: _) -> Result<(), Error>;
}
