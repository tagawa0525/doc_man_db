use crate::{AppState, graphql::create_schema};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{extract, response::Html};

/// GraphQL Playgroundの表示
pub async fn graphql_playground() -> Html<String> {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/graphql").title("Document Management GraphQL Playground"),
    ))
}

/// GraphQL クエリ/ミューテーション処理
pub async fn graphql_handler(
    extract::State(state): extract::State<AppState>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let schema = create_schema();
    schema.execute(req.into_inner().data(state)).await.into()
}
