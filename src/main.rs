#[allow(warnings)]
#[allow(unused)]


pub mod darkscout;
pub mod webapi;
use std::env;

use darkscout::types::{
    store::{NewDb, PgStore},
    SettingsEnv,
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let settings_env = SettingsEnv::from_file().await;
        let store: PgStore = PgStore::new(&settings_env.config).await;
        let command = &args[1];
        if command == "migrate" {
            match sqlx::migrate!("./migrations").run(&store.db).await {
                Ok(_) => {
                    println!("Migration Successful");
                    return;
                }
                Err(err) => {
                    println!("Doing migration");
                    dbg!(err);
                    return;
                }
            }
        }
    }
    // Start the tracer
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // let app = Router::new();
    let app = webapi::web_api().await;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
