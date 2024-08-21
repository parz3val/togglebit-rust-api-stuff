use std::collections::HashMap;

use crate::darkscout::adapters::{DSProvider, DarkSearchProvider};
use axum::http::{HeaderMap, HeaderValue};
use mongodb::bson::doc;
// use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use reqwest::{Client, Url};
// use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
// use serde::{Deserialize, Serialize};
use super::types::DarkSearchResponse;
use moka::future::{self};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const BREACH_URL: &'static str = "https://localhost:8002/darkweb/a/search/v1/search";

#[derive(Serialize, Deserialize, Clone)]
pub struct Cachedds_darkengineStats {
    pub _id: String, // id or domain
    pub breach_stats: DarkSearchResponse,
    pub added_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone)]
pub struct DarkSearchClient {
    client: Client,
    l1_cache: future::Cache<String, DarkSearchResponse>,
    l2_cache: mongodb::Collection<Cachedds_darkengineStats>,
}

impl DarkSearchProvider for DarkSearchClient {
    fn new(key: &str, l2_cache: mongodb::Client) -> Self {
        let mut headers = HeaderMap::new();
        let auth = format!("apikey {}", key);
        headers.insert(
            "Authorization",
            HeaderValue::from_str(auth.as_str()).unwrap(),
        );
        headers.insert("User-Agent", HeaderValue::from_static("DarkScout Web APP"));

        // // Authorization: apikey <API_KEY>
        // // Content-Type: application/json; charset=utf-8
        // // Accept: application/json; charset=utf-8
        headers.insert(
            "Content-Type",
            HeaderValue::from_static("application/json; charset=utf-8"),
        );
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json; charset=utf-8"),
        );

        // unwrapping because we want to panic if we can't build the client
        let client = Client::builder().default_headers(headers).build().unwrap();
        let l1_cache = future::Cache::builder()
            .time_to_live(Duration::from_secs(60 * 60 * 24))
            .build();
        let l2_cache = l2_cache
            .database("darkscout_l3_cache")
            .collection("ds_darkengine_stats");
        Self {
            client,
            l1_cache,
            l2_cache,
        }
    }

    async fn get_stats_by_email(
        &self,
        email: &str,
    ) -> std::result::Result<DarkSearchResponse, reqwest::Error> {
        if let Some(data) = self.l1_cache.get(email).await {
            println!("L1 Cache hit Success");
            return Ok(data);
        }

        // Check l2 Cache
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

        let mut body = HashMap::new();
        body.insert("query", email);

        let resp = self.client.post(BREACH_URL).json(&body).send().await?;

        // these two are separated so it is easier to debug when error decoding the response happens
        match resp.json::<DarkSearchResponse>().await {
            Ok(res) => {
                // insert into l1 cache
                self.l1_cache.insert(String::from(email), res.clone()).await;

                let result = Cachedds_darkengineStats {
                    _id: String::from(email),
                    breach_stats: res.clone(),
                    added_at: chrono::Utc::now(),
                };
                // insert into l2 cache
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
                return Err(e);
            }
        };
    }

    async fn get_stats_by_domain(
        &self,
        domain: &str,
    ) -> std::result::Result<DarkSearchResponse, reqwest::Error> {
        if let Some(data) = self.l1_cache.get(domain).await {
            println!("L1 Cache hit Success");
            return Ok(data);
        }

        // Check l2 Cache
        match self.l2_cache.find_one(doc! { "_id": domain }).await {
            Ok(Some(data)) => {
                println!("L2 Cache hit Success");
                let breach_info = data.breach_stats.clone();
                self.l1_cache
                    .insert(domain.to_string(), breach_info.clone())
                    .await;
                return Ok(breach_info);
            }
            Ok(None) => {}
            Err(_) => {}
        }
        let mut body = HashMap::new();

        body.insert("query", domain);

        let resp = self.client.post(BREACH_URL).json(&body).send().await?;

        match resp.json::<DarkSearchResponse>().await {
            Ok(res) => {
                // insert into l1 cache
                self.l1_cache
                    .insert(String::from(domain), res.clone())
                    .await;

                let result = Cachedds_darkengineStats {
                    _id: String::from(domain),
                    breach_stats: res.clone(),
                    added_at: chrono::Utc::now(),
                };
                // insert into l2 cache
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
                return Err(e);
            }
        };
    }
}
