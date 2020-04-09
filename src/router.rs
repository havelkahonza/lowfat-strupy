use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{RwLock, Arc};

use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::body::Bytes;
use hyper::service::{make_service_fn, service_fn};
use log::{debug, info};
use once_cell::sync::Lazy;

use crate::payload::Payload;

pub(crate) struct Router {
    payload: Arc<Payload>
}

impl Router {
    pub fn new(payload_manager: Arc<Payload>) -> Self {
        Router { payload: payload_manager }
    }

    pub async fn route(&self, request: Request<Body>) -> Response<Body> {
        match (request.method(), request.uri().path()) {
            (&Method::GET, "/update") => {
                debug!("Update");
                Response::builder()
                    .status(StatusCode::OK)
                    .body(Body::from(self.payload.read()))
                    .unwrap()
            }
            (&Method::POST, "/publish") => {
                debug!("Publish");
                match hyper::body::to_bytes(request.into_body()).await {
                    Ok(payload) => {
                        self.payload.write(payload);
                        Response::builder()
                            .status(StatusCode::OK)
                            .body(Body::from("Payload received"))
                            .unwrap()
                    }
                    _ => {
                        Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::empty()).unwrap()
                    }
                }
            }
            _ => Response::builder().status(StatusCode::NOT_FOUND).body(Body::empty()).unwrap()
        }

    }
}