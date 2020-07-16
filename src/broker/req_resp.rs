use crate::broker::topics::Status;
use crate::common::sled_db::*;
use crate::common::tools::*;
use actix_web::web;
use serde::{Deserialize, Serialize};
use std::{thread, time};
#[derive(Serialize, Deserialize, Debug)]
pub struct Req {
    #[serde(default = "default_string")]
    pub topic: String,
    #[serde(default = "default_string")]
    pub data: String,
    // timeout,length,takeout times
    #[serde(default = "default_i64")]
    pub timeout: i64,
}

impl Req {
    pub fn get(topic: &str) -> anyhow::Result<String> {
        let s = get_or_empty(topic);
        // 心跳拿到完了就删除,保证就拿一次
        if s != "".to_owned() {
            dbg!("hb ===={}", &s);
            remove(topic);
        }
        Ok(s)
    }
    pub fn save(&self) -> anyhow::Result<String> {
        // req仅仅保存data部分，topic作为k，time是请求方需要的
        let topic = &self.topic;
        let data = &self.data;
        insert(topic, data);
        Ok(SUCCESS.to_owned())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resp {
    #[serde(default = "default_string")]
    pub topic: String,
    // timeout,length,takeout times
    #[serde(default = "default_string")]
    pub answer: String,
}

impl Resp {
    pub fn init(_topic: &str) -> Self {
        Resp {
            topic: _topic.to_owned(),
            answer: "".to_owned(),
        }
    }
    pub fn resp_str(topic: &str) -> String {
        let mut resp = topic.to_owned();
        resp.push_str(SUFFIX_RESP);
        resp
    }

    pub fn save(&self) -> anyhow::Result<String> {
        let resp = serde_json::to_string(&self)?;
        let k = Resp::resp_str(&self.topic);
        insert(&k, &resp);
        Ok(SUCCESS.to_string())
    }
    pub fn get_reps(&mut self) -> anyhow::Result<bool> {
        let k = Resp::resp_str(&self.topic);
        let res = get_or_empty(&k);
        if res == "" {
            return Ok(false);
        }
        let data: Resp = serde_json::from_str(&res)?;
        self.answer = data.answer;
        // 拿到就删除
        remove(&k);
        Ok(true)
    }
}

pub async fn req_handler(msg: web::Json<Req>) -> anyhow::Result<String> {
    dbg!("req");
    dbg!(&msg);
    let req = msg.into_inner();
    let topic = &req.topic;
    let timeout = &req.timeout;
    req.save()?;

    let now = time_now_str();
    let mut resp = Resp::init(topic);
    let mut differ: i64 = 0;
    let one_millis = time::Duration::from_millis(1);
    while differ < *timeout {
        thread::sleep(one_millis);
        let get_answer = resp.get_reps()?;
        if get_answer {
            let res = resp.answer;
            return Ok(res);
        }
        differ = time_differ(&now);
    }

    let k = Resp::resp_str(topic);
    remove(&k);

    Status::feed_fail("time out")
}

pub fn heart_beat_handler(tp: &str) -> anyhow::Result<String> {
    Req::get(tp)
}
pub fn resp_handler(msg: web::Json<Resp>) -> anyhow::Result<String> {
    dbg!("resp");
    dbg!(&msg);
    let resp = msg.into_inner();
    resp.save()
}