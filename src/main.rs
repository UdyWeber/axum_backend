use std::net::SocketAddr;

mod router;

use router::router::mount_router;



#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = mount_router().await;
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


struct Testimonial<'s>{
    pub name: &'s str,
    pub comment: &'s str,
}

