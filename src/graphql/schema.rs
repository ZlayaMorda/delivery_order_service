use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use crate::AppState;
use crate::graphql::query::root::RootQuery;


pub type AppSchema = Schema<RootQuery, EmptyMutation, EmptySubscription>;

pub async fn build_schema(state: AppState) -> AppSchema {
    Schema::build(RootQuery::default(), EmptyMutation, EmptySubscription)
        .data(state)
        .finish()
}