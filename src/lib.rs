extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;
extern crate tokio_fs;
extern crate tokio_io;
extern crate rand;
#[macro_use]
extern crate lazy_static;


use hyper::{Body, Response};
use std::io;
use futures::{Future};

/*
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use ffjob::FfJob;
use hyper::service::service_fn;
*/
//mod playlist;
//mod segment;
pub mod ffjob;
pub mod transcode_task;
pub mod transcode_store;
//type ResponseFuture = Box<dyn Future<Item=Response<Body>, Error=io::Error> + Send>;

lazy_static! {
    static ref STORE: transcode_store::TranscodeStore = transcode_store::TranscodeStore::new();
}

/*

fn _main() {
    let j = FfJob::new_job();
    println!("{:?}", j);


    pretty_env_logger::init();
    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn(sitemap))
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    hyper::rt::run(server);

    crate::transcode_store::TranscodeStore::new();
    crate::transcode_task::TranscodeTask::new();
}


fn sitemap(req: Request<Body>) -> ResponseFuture {
    if req.method() == &Method::GET {
        let path = req.uri().path();
        if path.ends_with(".m3u8") {
            return crate::playlist::handle_playlist(req)
        } else if path.ends_with(".ts") {
            return crate::segment::handle_segment(req)
        } else {
            return Box::new(future::ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap()))
        }
    } else {
        return Box::new(future::ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()))
    }


}

*/