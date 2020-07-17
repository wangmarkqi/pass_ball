use crate::broker::req_resp::{heart_beat_handler, req_handler, resp_handler, Req, Resp};
use crate::broker::subpub::{
    conf_sub_handler, publish_handler, subscribe_handler, ConfSub, SubPub,
};
use crate::broker::topics::Status;
use crate::broker::topics::{del_topics_handler, get_topics_handler};
use actix_web::{web, HttpMessage, HttpRequest, Result};

fn anyhow2str(res: anyhow::Result<String>) -> String {
    match res {
        Ok(s) => return s,
        Err(e) => {
            let s = e.to_string();
            Status::feed_fail(&s).unwrap()
        }
    }
}

pub async fn conf_sub(conf: web::Query<ConfSub>) -> Result<String> {
    Ok(anyhow2str(conf_sub_handler(conf)))
}

pub async fn subscribe(req: HttpRequest) -> Result<String> {
    let topic = req.match_info().get("topic").unwrap_or("");
    Ok(anyhow2str(subscribe_handler(topic)))
}

pub async fn publish(msg: web::Json<SubPub>) -> Result<String> {
    Ok(anyhow2str(publish_handler(msg)))
}

// 手动删除topic
pub async fn get_topics() -> Result<String> {
    Ok(anyhow2str(get_topics_handler()))
}

pub async fn del_topic(req: HttpRequest) -> Result<String> {
    let topic = req.match_info().get("topic").unwrap_or("");
    Ok(anyhow2str(del_topics_handler(topic)))
}

// another mod req-reply ,the heartbeat is get req
pub async fn req(msg: web::Json<Req>) -> Result<String> {
    let res = req_handler(msg).await;
    Ok(anyhow2str(res))
}

pub async fn heart_beat(req: HttpRequest) -> Result<String> {
    let topic = req.match_info().get("topic").unwrap_or("");
    Ok(anyhow2str(heart_beat_handler(topic)))
}

pub async fn resp(msg: web::Json<Resp>) -> Result<String> {
    Ok(anyhow2str(resp_handler(msg)))
}
