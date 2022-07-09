use actix::{Actor, Addr};
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, web::{Data, Payload, ServiceConfig, get}};
use actix_web::web::route;
use actix_web_actors::ws;
use uuid::Uuid;

mod session;
mod server;
mod messages;
mod room;
mod protocol;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = server::GameServer::default().start();
    let app = move || App::new().app_data(Data::new(server.clone())).route("/ws", get().to(index));
    HttpServer::new(app).bind(("127.0.0.1", 8080))?.run().await
}

async fn index(req: HttpRequest, stream: Payload, srv: Data<Addr<server::GameServer>>) -> Result<HttpResponse, Error> {
    ws::start(session::Session::new(Uuid::nil(), srv.get_ref().clone()), &req, stream)
}
