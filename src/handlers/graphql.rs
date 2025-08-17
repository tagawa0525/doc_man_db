use axum::{response::Html, Json};

/// GraphQL Playgroundの表示
pub async fn graphql_playground() -> Html<&'static str> {
    Html(
        r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset="utf-8" />
        <title>GraphQL Playground</title>
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/static/css/index.css" />
        <link rel="shortcut icon" href="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/favicon.png" />
        <script src="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/static/js/middleware.js"></script>
    </head>
    <body>
        <div id="root">
            <style>
                body { background-color: rgb(23, 42, 58); }
                #root { height: 100vh; }
            </style>
        </div>
        <script>window.addEventListener('load', function (event) {
            GraphQLPlayground.init(document.getElementById('root'), { endpoint: '/graphql' })
        })</script>
    </body>
    </html>
    "#,
    )
}

/// GraphQL クエリ/ミューテーション処理
pub async fn graphql_handler(
    Json(_request): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    // TODO: GraphQL実装
    Ok(Json(serde_json::json!({
        "data": null,
        "errors": [{"message": "GraphQL implementation pending"}]
    })))
}