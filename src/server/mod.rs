use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Response, Server};

const TEXT: &str = "Good, sssss!";

fn server() {
    let addr = ([127, 0, 0, 1], 8080).into();

    let new_svc = || service_fn_ok(|_req| Response::new(Body::from(TEXT)));

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));
    println!("Developping Server Listenning... ");

    hyper::rt::run(server);
}
