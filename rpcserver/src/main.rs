use core_rpc::run_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_max_level(tracing::Level::DEBUG)
        .try_init()
        .expect("setting default subscriber failed");

    // Start up a JSON-RPC server that allows cross origin requests.
    let server_addr = run_server().await?;

    // Print instructions for testing CORS from a browser.
    tracing::info!("Server started at: {}", server_addr);
    tracing::info!(
        r#"try me: 
                      $ curl -X POST http://{} \
                        -H "Content-Type: application/json" \
                        -d '{{ "jsonrpc": "2.0", "method": "say_hello", "id": 1 }}' "#,
        server_addr
    );

    futures::future::pending().await
}
