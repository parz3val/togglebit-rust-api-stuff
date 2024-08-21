use crate::darkscout::adapters::DSProvider;
use axum::http::{HeaderMap, HeaderValue};
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use moka::future::{self};
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use serde::{Deserialize, Serialize};
pub mod api;
use mongodb::bson::doc;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BreachName {
    pub name: String,
    pub title: String,
    pub description: String,
    pub domain: Option<String>,
    pub breach_date: Option<String>,
    pub added_date: Option<String>,
    pub modified_date: Option<String>,
    pub pwn_count: Option<u64>,
    pub data_classes: Option<Vec<String>>,
    pub logo_path: String,
    pub is_verified: Option<bool>,
    pub is_fabricated: Option<bool>,
    pub is_sensitive: Option<bool>,
    pub is_retired: Option<bool>,
    pub is_spamlist: Option<bool>,
    pub is_malicious_verified: Option<bool>,
    pub is_subscription_free: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CachedBreachedStats {
    pub _id: String,
    pub breach_stats: Vec<BreachName>,
    pub added_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone)]
pub struct BreachClient {
    client: Client,
    l1_cache: future::Cache<String, Vec<BreachName>>,
    l2_cache: mongodb::Collection<CachedBreachedStats>,
}

impl DSProvider for BreachClient {
    async fn new(
        key: &'static str,
        mongo: mongodb::Client,
    ) -> std::result::Result<BreachClient, reqwest::Error> {
        let mut headers = HeaderMap::new();
        headers.insert("hibp-api-key", HeaderValue::from_static(key));
        headers.insert("user-agent", HeaderValue::from_static("dark-scout-app"));
        let l2_cache = mongo
            .database("darkscout_l3_cache")
            .collection("hibp_breached_stats");
        let client = Client::builder().default_headers(headers).build().unwrap();
        let l1_cache = future::Cache::builder()
            .time_to_live(Duration::from_secs(60 * 60 * 24))
            .build();
        Ok(BreachClient {
            client,
            l1_cache,
            l2_cache,
        })
    }

    async fn get_stats_by_email(
        &self,
        email: &str,
    ) -> std::result::Result<Vec<BreachName>, reqwest_middleware::Error> {
        if let Some(data) = self.l1_cache.get(email).await {
            println!("L1 Cache hit Success");
            return Ok(data);
        }

        // Check L2 Cache
        match self.l2_cache.find_one(doc! { "_id": email }).await {
            Ok(Some(data)) => {
                println!("L2 Cache hit Success");
                let breach_info = data.breach_stats.clone();
                self.l1_cache
                    .insert(email.to_string(), breach_info.clone())
                    .await;
                return Ok(breach_info);
            }
            Ok(None) => {}
            Err(_) => {}
        }

        const BREACH_URL: &'static str = "http://localhost:8001/searchapi/accounts";
        let url = format!("{}/{}?truncateResponse=false", BREACH_URL, email);

        match self
            .client
            .get(url)
            .send()
            .await?
            .json::<Vec<BreachName>>()
            .await
        {
            Ok(res) => {
                // insert into cache
                self.l1_cache.insert(String::from(email), res.clone()).await;

                // insert into l3 cache
                let result = CachedBreachedStats {
                    _id: String::from(email),
                    breach_stats: res.clone(),
                    added_at: chrono::Utc::now(),
                };
                let insert_closure = self.l2_cache.insert_one(result).await;

                // spawn a task to insert into l2 cache
                tokio::spawn(async move {
                    if let Err(e) = insert_closure {
                        tracing::debug!("Error inserting data into l3 cache: {}", e);
                    }
                });

                return Ok(res);
            }
            Err(e) => {
                return Err(reqwest_middleware::Error::Middleware(e.into()));
            }
        };
    }

    async fn get_stats_by_domain(
        &self,
        domain: &str,
    ) -> std::result::Result<(), reqwest_middleware::Error> {
        /**
        **/
        const BREACH_URL: &'static str = "http://localhost:8001/searchapi/domains";
        let url = format!("{}/{}", BREACH_URL, domain);
        return match self.client.get(url).send().await?.text().await {
            Ok(res) => {
                println!("{}", res);
                Ok(())
            }
            Err(e) => Err(reqwest_middleware::Error::Middleware(e.into())),
        };
    }
}