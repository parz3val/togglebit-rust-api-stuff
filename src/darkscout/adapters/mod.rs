use std::result::Result;
pub mod dsbreach;
pub mod smtp_mailer;
pub mod ds_darkengine;

use crate::darkscout::adapters::dsbreach::BreachName;
use crate::darkscout::adapters::ds_darkengine::types::DarkSearchResponse;

#[allow(async_fn_in_trait)]
pub trait DSProvider {
    async fn new(key: &'static str, mongo: mongodb::Client) -> Result<Self, reqwest::Error>
    where
        Self: Sized;
    async fn get_stats_by_email(
        &self,
        email: &str,
    ) -> Result<Vec<BreachName>, reqwest_middleware::Error>;
    async fn get_stats_by_domain(&self, domain: &str) -> Result<(), reqwest_middleware::Error>;
}

#[allow(async_fn_in_trait)]
pub trait DarkSearchProvider {
    fn new(key: &str, l2_cache: mongodb::Client) -> Self
    where
        Self: Sized;
    async fn get_stats_by_email(&self, email: &str) -> Result<DarkSearchResponse, reqwest::Error>;
    async fn get_stats_by_domain(&self, domain: &str)
        -> Result<DarkSearchResponse, reqwest::Error>;
}


#[allow(async_fn_in_trait)]
pub trait SendGridProvider {
    fn new(key: &str)-> Self where Self: Sized;
    async fn send(&self, _: String) -> Result<(), reqwest::Error>;
}
