use std::{net::SocketAddr, sync::Arc};

use axum::{
    routing::{get, on, MethodFilter},
    Extension, Router,
};
use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};
use juniper_axum::{graphql, playground};
use tokio::net::TcpListener;

#[derive(Clone, Copy, Debug)]
pub struct Query;

#[graphql_object]
impl Query {
    fn hello() -> String {
        "Hello, World!".to_string()
    }
}

type Schema = RootNode<'static, Query, EmptyMutation, EmptySubscription>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let schema = Schema::new(Query, EmptyMutation::new(), EmptySubscription::new());

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/graphql",
            on(
                MethodFilter::GET.or(MethodFilter::POST),
                graphql::<Arc<Schema>>,
            ),
        )
        .route("/playground", get(playground("/graphql", "/subscriptions")))
        .layer(Extension(Arc::new(schema)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("listening on {addr}");

    axum::serve(listener, app).await?;

    Ok(())
}
