pub mod broker;
pub mod common;

use crate::common::tools::get_dot_env;
use actix_web::{web, App, HttpServer};
use broker::router;

use crate::broker::req_resp::{Req, Resp};

#[macro_use]
extern crate anyhow;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let url = get_dot_env("URL");
    HttpServer::new(|| {
        App::new()
            // Set max file size to 6 Mib
            .app_data(web::PayloadConfig::new(1024 * 1024 * 1024 * 1024))
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
