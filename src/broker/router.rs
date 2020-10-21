use crate::broker::req_resp::{req_handler, Req, Resp};
use crate::broker::subpub::{
    conf_sub_handler, publish_handler, subscribe_handler, ConfSub, SubPub,
};
use crate::broker::topics::Status;
use crate::broker::topics::{del_topics_handler, get_topics_handler};
use actix_web::{get, post, web, HttpRequest, Result};
use bytes::Bytes;

fn anyhow2str(res: anyhow::Result<String>) -> String {
    match res {
        Ok(s) => return s,
        Err(e) => {
            let s = e.to_string();
            Status::feed_fail(&s).unwrap()
        }
    }
}
// 通过?topic=&timeout=&len=&consume=的方式，参数多
#[get("/pass/conf")]
pub async fn conf_sub(conf: web::Query<ConfSub>) -> Result<String> {
    Ok(anyhow2str(conf_sub_handler(conf)))
}

#[get("/pass/sub/{topic}")]
pub async fn subscribe(myreq: HttpRequest) -> Result<String> {
    let topic = myreq.match_info().get("topic").unwrap_or("");
    Ok(anyhow2str(subscribe_handler(topic)))
}

#[post("/pass/pub/{topic}")]
pub async fn publish(myreq: HttpRequest, body: Bytes) -> Result<String> {
    let _topic = myreq.match_info().get("topic").unwrap_or("");
    let result = std::str::from_utf8(&body).unwrap(); // return Resul

    let subpub = SubPub {
        topic: _topic.to_string(),
        answer: result.to_string(),
        time: "".to_string(),
    };
    Ok(anyhow2str(publish_handler(subpub)))
}

#[get("/pass/topics")]
pub async fn get_topics() -> Result<String> {
    Ok(anyhow2str(get_topics_handler()))
}

// 手动删除topic
#[get("/pass/del/{topic}")]
pub async fn del_topic(myreq: HttpRequest) -> Result<String> {
    let topic = myreq.match_info().get("topic").unwrap_or("");
    Ok(anyhow2str(del_topics_handler(topic)))
}

// another mod req-reply ,the heartbeat is get req
#[post("/pass/req/{topic}/{timeout}")]
pub async fn req(info: web::Path<(String, i64)>, body: Bytes) -> Result<String> {
    let result = std::str::from_utf8(&body).unwrap();
    let _topic = &info.0;
    dbg!("get req");
    dbg!(result);
    let r = Req {
        topic: _topic.to_string(),
        data: result.to_string(),
        timeout: info.1,
    };
    let res = req_handler(r).await;
    Ok(anyhow2str(res))
}

#[get("/pass/hb/{topic}")]
pub async fn heart_beat(myreq: HttpRequest) -> Result<String> {
    let topic = myreq.match_info().get("topic").unwrap_or("");
    let res = Req::get(topic);
    Ok(anyhow2str(res))
}

#[post("/pass/resp/{topic}")]
pub async fn resp(myreq: HttpRequest, body: Bytes) -> Result<String> {
    let _topic = myreq.match_info().get("topic").unwrap_or("");
    let result = std::str::from_utf8(&body).unwrap(); // return Resul
    let resp = Resp {
        topic: _topic.to_string(),
        answer: result.to_string(),
    };
    let res = resp.save();
    Ok(anyhow2str(res))
}
