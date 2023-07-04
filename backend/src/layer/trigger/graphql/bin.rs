use mensa_app_backend::layer::trigger::graphql::{server::GraphQLServerInfo, *};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    // setup logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_writer(std::io::stderr)
        .pretty()
        // .with_env_filter(EnvFilter::default())
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // run server
    let info = GraphQLServerInfo { port: 8090 };

    let mut server = server::GraphQLServer::new(info, mock::RequestDatabaseMock, mock::CommandMock);
    server.start();
    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for CTRL-C event");
    info!("stopping server...");
    server.shutdown().await;
}
