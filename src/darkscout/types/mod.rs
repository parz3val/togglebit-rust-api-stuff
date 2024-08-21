use self::store::PgStore;
use crate::darkscout::adapters::dsbreach::BreachClient;
use lettre::SmtpTransport;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use store::DSCache;
use tokio::io::AsyncReadExt;

use super::adapters::ds_darkengine::api::DarkSearchClient;

pub mod errors;
pub mod invitations;
pub mod member;
pub mod store;
pub mod user;
pub mod workspace;
pub mod verified_emails;
pub mod verified_domains;

pub mod auth;
pub mod darkmonitor;

#[derive(Clone, Deserialize, Serialize)]
pub struct Settings {
    pub app_url: String,
    pub api_url: String,
    pub db_uri: String,
    pub mdb_uri: String,
    pub cache_uri: String,
    pub dsbreach_api_key: String,
    pub ds_darkengine_api_key: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct JwtSettings {
    pub jwt_secret: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct SmtpSettings {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct SettingsEnv {
    pub config: Settings,
    pub app: Application,
    pub jwt: JwtSettings,
    pub smtp: SmtpSettings,
}

impl SettingsEnv {
    async fn file_to_string(file: &str) -> String {
        let file = tokio::fs::File::open(file).await.unwrap();
        let mut buf_reader = tokio::io::BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).await.unwrap();
        return contents;
    }
    pub async fn from_file() -> Self {
        match toml::from_str(
            Self::file_to_string("src/webapi/config/development.toml")
                .await
                .as_str(),
        ) {
            Ok(data) => return data,
            Err(e) => {
                dbg!("{:?}", e);
                panic!("Error")
            }
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Application {
    pub version: f32,
}

#[derive(Clone)]
pub struct AppState {
    pub settings: SettingsEnv,
    pub db: Arc<PgStore>,
    pub cache: Arc<DSCache>,
    pub ds_provider: BreachClient,
    pub ds_darkengine_provider: DarkSearchClient,
    pub smtp_client: SmtpTransport,
}

#[derive(Serialize, Deserialize)]
pub struct DSResponse<T> {
    pub data: Option<T>,
    pub err: Option<&'static str>,
}

impl<T> DSResponse<T> {
    pub fn new(data: Option<T>, err: Option<&'static str>) -> Self {
        Self { data, err }
    }
}
