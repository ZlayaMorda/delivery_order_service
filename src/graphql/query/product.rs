use async_graphql::Object;

#[derive(Default)]
pub struct ProductQuery;

#[Object]
impl ProductQuery {
    async fn get_test(&self) -> bool {
        true
    }
}