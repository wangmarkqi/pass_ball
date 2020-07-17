pub mod broker;
pub mod common;

use crate::common::tools::get_dot_env;
use actix_web::{App, HttpServer};
use broker::router;
#[macro_use]
extern crate anyhow;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let url = get_dot_env("URL");
    HttpServer::new(|| {
        App::new()
            .service(router::conf_sub)
            .service(router::subscribe)
            .service(router::publish)
            .service(router::get_topics)
            .service(router::del_topic)
            .service(router::req)
            .service(router::heart_beat)
            .service(router::resp)
    })
    .bind(url)?
    .run()
    .await
}
