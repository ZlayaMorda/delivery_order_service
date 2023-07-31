use axum::{Router};
use axum::routing::get;
use tracing::{Subscriber};
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{EnvFilter, Registry};
use tracing_subscriber::layer::SubscriberExt;
use crate::db::utils::get_pool;
use crate::utils::config::Config;
use crate::utils::errors::AppError;

pub mod utils;
pub mod db;

pub fn get_subscriber(
    config: &Config
) -> impl Subscriber + Send + Sync {

    let env_filter = EnvFilter::new(config.tracing_filter.to_string());

    let file_appender = tracing_appender::rolling::hourly(
        format!("{}/{}-logs", &config.logs_path, &config.name),
        format!("{}.log", &config.name)
    );

    let formatting_layer = BunyanFormattingLayer::new(
        config.name.to_string(),
        file_appender
    );

    Registry::default()
    .with(env_filter)
    .with(JsonStorageLayer)
    .with(tracing_subscriber::fmt::layer())
    .with(formatting_layer)
}

pub async fn run(mode: &str) -> Result<(), AppError> {
    let config = Config::init(mode);
    let socket = format!("{}:{}", config.service_host, config.service_port);

    let tracing_subscriber = get_subscriber(&config);
    set_global_default(tracing_subscriber).expect("Error while set tracing subscriber");

    let pool = get_pool(&config.db_url).await?;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(pool);

    let server = axum::Server::bind(&socket.parse().expect("dsf"))
        .serve(app.into_make_service())
        .await
        .expect("Error while start server");

    Ok(server)
}