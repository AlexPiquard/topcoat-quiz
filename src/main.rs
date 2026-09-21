mod app;
mod components;
mod quiz;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let router = app::router();
    topcoat::start(router).await.unwrap();
}
