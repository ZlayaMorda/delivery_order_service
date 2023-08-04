use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql::Schema;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{Extension, Router};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use sqlx::{Pool, Postgres};
use tracing::{Subscriber};
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{EnvFilter, Registry};
use tracing_subscriber::layer::SubscriberExt;
use crate::db::utils::get_pool;
use crate::graphql::schema::{AppSchema, build_schema};
use crate::utils::config::Config;
use crate::utils::errors::AppError;

pub mod utils;
pub mod db;
pub mod models;
pub mod graphql;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
    pub env: Config,
}

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

async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/api/graphql"
    )))
}

pub async fn run(mode: &str) -> Result<(), AppError> {
    let config = Config::init(mode);
    let socket = format!("{}:{}", config.service_host, config.service_port);

    let tracing_subscriber = get_subscriber(&config);
    set_global_default(tracing_subscriber).expect("Error while set tracing subscriber");

    let pool = get_pool(&config.db_url).await?;

    let app_state = AppState {
        db: pool.clone(),
        env: config.clone(),
    };

    let schema = build_schema().await;

    let app = Router::new()
        .route(
            "/api/graphql",
            get(graphql_playground).post(graphql_handler)
        )
        .layer(Extension(schema))
        .with_state(app_state);

    let server = axum::Server::bind(&socket.parse().expect("Error while parsing socket"))
        .serve(app.into_make_service())
        .await
        .expect("Error while start server");

    Ok(server)
}