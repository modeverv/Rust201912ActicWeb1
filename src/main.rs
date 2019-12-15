use actix_web::{server,App,HttpRequest,Responder,Path, Error,FromRequest,State};
use serde_derive::*;

fn hello<S>(req:&HttpRequest<S>) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!",to)
}

#[derive(Deserialize)]
struct HelloPath {
    name: String,
}

fn hello_name(to: Path<HelloPath>)->impl Responder {
    format!("Hello {} !",&to.name)
}

struct MyApp {
    server_name: String,
}

fn hello_with_state(app: State<MyApp>)->Result<String,Error> {
    Ok(format!("Hello from {}!", &app.server_name))
}

fn main() {
    server::new(|| {
        App::with_state(MyApp{
            server_name: "server with state".into(),
        })
//        .resource("/", |r| r.f(hello))
        .resource("/", |r| r.with(hello_with_state))        
//        .resource("/{name}", |r| r.f(hello))
        .resource("/{name}", |r| r.with(hello_name))        
    })
    .bind("localhost:3000")
    .expect("Can not bind to port 3000")
    .run();
}
