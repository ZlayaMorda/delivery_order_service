use async_graphql::{EmptySubscription, Schema};
use async_graphql::extensions::{Tracing};
use crate::AppState;
use crate::graphql::mutation::root::RootMutation;
use crate::graphql::query::root::RootQuery;


pub type AppSchema = Schema<RootQuery, RootMutation, EmptySubscription>;

pub async fn build_schema(state: AppState) -> AppSchema {
    Schema::build(RootQuery::default(), RootMutation::default(), EmptySubscription)
        .data(state)
        .extension(Tracing)
        .finish()
}