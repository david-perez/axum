//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```

use axum::http::HeaderMap;
use axum::{handler::Handler, response::Html};
use hyper::{service::make_service_fn, Body, Request, Response, StatusCode};
use std::net::SocketAddr;

pub struct MyService;

static NOT_FOUND: &[u8] = b"Not Found";

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    let make_service = make_service_fn(move |_| {
        async move {
            Ok::<_, std::convert::Infallible>(hyper::service::service_fn(move |mut req| {
                async move {
                    dbg!(&req);

                    let resp = Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(hyper::Body::from(NOT_FOUND))
                        .unwrap();
                    Ok::<_, std::convert::Infallible>(resp)

                    // Ok::<_, std::convert::Infallible>(
                    //     Response::builder().status(StatusCode::NOT_FOUND).body(hyper::Body::from(NOT_FOUND)).unwrap(),
                    // )
                    // Ok::<_, std::convert::Infallible>(
                    //     match router.find(&path) {
                    //     Some((handler, params)) => {
                    //         let p = params.iter().map(|p| (p.0.to_string(), p.1.to_string())).collect::<Params>();
                    //         req.extensions_mut().insert(p);
                    //         handler.call(req).await
                    //     }
                    //     None => Response::builder().status(StatusCode::NOT_FOUND).body(NOT_FOUND.into()).unwrap(),
                    // })
                }
            }))
        }
    });

    axum::Server::bind(&addr).serve(make_service).await.unwrap();
}

async fn handler_wildcard(all_headers: HeaderMap) -> Html<String> {
    Html(format!("{:?}\n", all_headers))
}
