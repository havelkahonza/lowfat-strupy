use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};

use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::body::Bytes;
use hyper::service::{make_service_fn, service_fn};
use log::{debug, info};
use once_cell::sync::Lazy;

use crate::payload::Payload;
use crate::router::Router;

mod router;
mod payload;

// static PAYLOAD: Lazy<RwLock<Bytes>> = Lazy::new(|| RwLock::default());

// async fn router(request: Request<Body>) -> Response<Body> {
//     match (request.method(), request.uri().path()) {
//         (&Method::GET, "/update") => {
//             debug!("Update");
//             let payload = PAYLOAD.read().expect("rwlock read error").clone();
//             Response::builder()
//                 .status(StatusCode::OK)
//                 .body(Body::from(payload))
//                 .unwrap()
//         }
//         (&Method::POST, "/publish") => {
//             debug!("Publish");
//             match hyper::body::to_bytes(request.into_body()).await {
//                 Ok(payload) => {
//                     *PAYLOAD.write().expect("rwlock write error") = payload;
//                     Response::builder()
//                         .status(StatusCode::OK)
//                         .body(Body::from("Payload received"))
//                         .unwrap()
//                 }
//                 _ => {
//                     Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::empty()).unwrap()
//                 }
//             }
//         }
//         _ => Response::builder().status(StatusCode::NOT_FOUND).body(Body::empty()).unwrap()
//     }
// }



#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting the web server");

    let payload = Arc::new(Payload::new());
    let router = Arc::new(Router::new(payload));


    let server =
        Server::bind(&SocketAddr::from(([127, 0, 0, 1], 8080)))
            .serve(make_service_fn(move |_| async move {
                let router = router.clone();
                debug!("New connection has been started");
                Ok::<_, Infallible>(service_fn(move |req| async move {
                    let router = router.clone();
                    Ok::<_, Infallible>(router.route(req).await)
                }))
            }));

    info!("Server has started");
    // Run this server for... forever!
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    // Run this server for... forever!
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

// async fn streaming_updates_service(request: Request<Body>) -> Result<Response<Body>, Infallible> {
//     Ok(router(request).await)
// }
