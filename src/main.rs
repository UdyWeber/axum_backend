use std::{net::SocketAddr, env::var};

mod db;
mod models;
mod router;
mod schema;

use router::router::mount_router;

use crate::db::establish_connection;

mod generics {
    use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
    pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let connection_pool = establish_connection().await;

    let port = var("SERVER_PORT").unwrap().parse::<u16>().unwrap();

    // build our application with a route
    let app = mount_router(connection_pool);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
