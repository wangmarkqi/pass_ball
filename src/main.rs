pub mod broker;
pub mod common;

use crate::common::tools::get_dot_env;
use actix_web::{web, App, HttpServer};
use broker::router;
#[macro_use]
extern crate anyhow;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let url = get_dot_env("URL");
    HttpServer::new(|| {
        App::new()
            .route("/pass/conf", web::get().to(router::conf_sub))
            .route("/pass/sub/{topic}", web::get().to(router::subscribe))
            .route("/pass/pub", web::post().to(router::publish))
            .route("/pass/topics", web::get().to(router::get_topics))
            .route("/pass/del/{topic}", web::get().to(router::del_topic))
            .route("/pass/req", web::post().to(router::req))
            .route("/pass/hb/{topic}", web::get().to(router::heart_beat))
            .route("/pass/resp", web::post().to(router::resp))
    })
    .bind(url)?
    .run()
    .await
}
