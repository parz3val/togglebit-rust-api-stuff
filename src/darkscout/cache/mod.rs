use std::time::Duration;
use super::types::{
    store::{DSCache, NewDb},
    Settings,
};
use moka::future::Cache;
use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};

//     Ok(())
// }
impl NewDb for DSCache {
    async fn new(settings: &Settings) -> Self {
        let uri: &str = settings.mdb_uri.as_ref();

        let mut client_options = ClientOptions::parse(uri).await.unwrap(); // we want to crash if we can't connect to our mdb

        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();

        client_options.server_api = Some(server_api);

        // Create a new client and connect to the server
        let client = Client::with_options(client_options).unwrap();

        // Send a ping to confirm a successful connection
        client
            .database("darkscout_l3_cache")
            .run_command(doc! { "ping": 1 })
            .await
            .unwrap();

        // Let's create a in memory cache for the l1 cache
        // For now we'll, only store 10k items
        // let l1_cache = Cache::new(10_000);
        let l1_cache = Cache::builder()
            // Time to live (TTL): 30 minutes
            .time_to_live(Duration::from_secs(60 * 60* 24))
            // Time to idle (TTI):  5 minutes
            // .time_to_idle(Duration::from_secs(5 * 60))
            // Create the cache.
            .build();
        return DSCache {
            l3: client,
            l1: l1_cache,
        };
    }
}


// DOCS
//
// use mongodb::{ bson::doc, options::{ ClientOptions, ServerApi, ServerApiVersion }, Client };
// #[tokio::main]
// async fn main() -> mongodb::error::Result<()> {
//     // Replace the placeholder with your Atlas connection string
//     let uri = "<connection string>";
//     let mut client_options = ClientOptions::parse_async(uri).await?;

//     // Set the server_api field of the client_options object to Stable API version 1
//     let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
//     client_options.server_api = Some(server_api);

//     // Create a new client and connect to the server
//     let client = Client::with_options(client_options)?;

//     // Send a ping to confirm a successful connection
//     client.database("admin").run_command(doc! { "ping": 1 }).await?;
//     println!("Pinged your deployment. You successfully connected to MongoDB!");

