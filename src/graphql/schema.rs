use async_graphql::{EmptySubscription, Schema};
use super::resolvers::{QueryRoot, MutationRoot};

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> AppSchema {
    Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .finish()
}