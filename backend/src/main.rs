
use actix_web::{App, HttpServer, web};

mod packets;
mod arp_scan;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/api/packets/get_interfaces", web::get().to(packets::get_interfaces))
            .route("/api/scan/discover_hosts/{interface_name}", web::get().to(arp_scan::discover_hosts))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}