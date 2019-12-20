//#![deny(warnings)]
extern crate futures;
extern crate hyper;

use hyper::{Body, Request, Response, StatusCode};
use futures::{future};
use crate::ResponseFuture;

pub fn handle_segment(_req: Request<Body>) -> ResponseFuture {
    return Box::new(future::ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap()))
}
