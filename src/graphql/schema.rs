use super::resolvers::{MutationRoot, QueryRoot};
use async_graphql::{EmptySubscription, Schema};

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish()
}
