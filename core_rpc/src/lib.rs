use hyper::Method;

use jsonrpsee::core::RpcResult;
use jsonrpsee::server::{RpcModule, Server};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

async fn handle_hello() -> RpcResult<&'static str> {
    Ok("Hello there!!")
}

pub async fn run_server() -> anyhow::Result<SocketAddr> {
    // Add a CORS middleware for handling HTTP requests.
    // This middleware does affect the response, including appropriate
    // headers to satisfy CORS. Because any origins are allowed, the
    // "Access-Control-Allow-Origin: *" header is appended to the response.
    let cors = CorsLayer::new()
        // Allow `POST` when accessing the resource
        .allow_methods([Method::POST])
        // Allow requests from any origin
        .allow_origin(Any)
        .allow_headers([hyper::header::CONTENT_TYPE]);
    let middleware = tower::ServiceBuilder::new().layer(cors);

    // The RPC exposes the access control for filtering and the middleware for
    // modifying requests / responses. These features are independent of one another
    // and can also be used separately.
    // In this example, we use both features.
    let server = Server::builder()
        .set_http_middleware(middleware)
        .build("0.0.0.0:0".parse::<SocketAddr>()?)
        .await?;

    let mut module = RpcModule::new(());
    module.register_async_method("say_hello", |_, _, _| async move { handle_hello().await })?;

    let addr = server.local_addr()?;
    let handle = server.start(module);

    // In this example we don't care about doing shutdown so let's it run forever.
    // You may use the `ServerHandle` to shut it down or manage it yourself.
    tokio::spawn(handle.stopped());

    Ok(addr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handle_hello() {
        let response = handle_hello().await.unwrap();
        assert_eq!(response, "Hello there!!");
    }
}
