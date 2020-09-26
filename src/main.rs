use std::fmt;
use futures::{future, Future};
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use hyper::service::service_fn;
use std::sync::{Arc, Mutex};
use slab::Slab;

const INDEX: &'static str = r#"
<!doctype html>
<html>
     <head>
          <title>Rust Microservice</title>
     </head>
     <body>
          <h3>Rust Microservice</h3>
     </body>
</html>
"#;

type UserId = u64;

struct UserData;

type UserDb = Arc<Mutex<Slab<UserData>>>;

fn microservice_handler(req: Request<Body>, user_db: &UserDb)
                        -> impl Future<Item=Response<Body>, Error=Error>
{
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            future::ok(Response::new(INDEX.into()))
        }
        _ => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap();
            future::ok(response)
        }
    }
}

fn main() {
    let addr = ([127, 0, 0, 1], 3030).into();
    let builder = Server::bind(&addr);
    let user_db = Arc::new(Mutex::new(Slab::new()));
    let server = builder.serve(move || {
        let user_db = user_db.clone();
        service_fn(move |req| microservice_handler(req, &user_db))
    });
    let server = server.map_err(drop);
    hyper::rt::run(server)
}


