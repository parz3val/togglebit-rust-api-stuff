pub mod invitations;
pub mod members;
pub mod users;
pub mod workspaces;
pub mod auth;
pub mod verified_emails;
pub mod verified_domains;

use std::sync::Arc;
use sqlx::postgres::PgPoolOptions;
use crate::darkscout::types::store::{AuthRepo, InvitationRepo, MemberRepo, UserRepo, WorkspaceRepo, VerifiedEmailsRepo, VerifiedDomainRepo};

use super::types::{
    store::{NewDb, PgStore},
    Settings,
};

impl NewDb for PgStore {
    async fn new(settings: &Settings) -> Self {
        println!("Connecting to the database {:?}", settings.db_uri.clone());
        let pool = match PgPoolOptions::new()
            .max_connections(1)
            .connect(settings.db_uri.as_ref())
            .await
        {
            Ok(pool) => pool,
            Err(e) => {
                println!(" Database connection Error: {:?}", e);
                panic!("Err");
            }
        };
        println!("Connected to the database!");
        PgStore {
            db: pool.clone(),
            user: Arc::new(UserRepo{ db: pool.clone() }),
            workspace: Arc::new(WorkspaceRepo { db: pool.clone() }),
            invitation: Arc::new(InvitationRepo { db: pool.clone() }),
            member: Arc::new(MemberRepo { db:  pool.clone()}),
            auth: Arc::new(AuthRepo { db: pool.clone()}),
            verified_emails: Arc::new(VerifiedEmailsRepo { db: pool.clone()}),
            verified_domains: Arc::new(VerifiedDomainRepo { db: pool.clone()})
        }
    }
}
