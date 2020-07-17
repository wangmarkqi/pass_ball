use std::collections::HashSet;

use actix_web::web;
use serde::{Deserialize, Serialize};

use crate::broker::topics::{Status, TopicManagement, Topics};
use crate::common::sled_db::*;
use crate::common::sled_db::{insert, read_set_from_db, update_set_from_db_and_str};
use crate::common::tools::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfSub {
    #[serde(default = "default_string")]
    pub topic: String,
    #[serde(default = "default_time")]
    pub timeout: i64,
    #[serde(default = "default_len")]
    pub len: i64,
    #[serde(default = "default_once")]
    pub consume: String,
}

fn default_once() -> String {
    "once".to_string()
}

fn default_time() -> i64 {
    24 * 60 * 60
}

fn default_len() -> i64 {
    100000
}

impl ConfSub {
    pub fn topic_conf_str(topic: &str) -> String {
        let mut topic_conf = topic.to_owned();
        topic_conf.push_str(SUFFIX_CONF);
        topic_conf
    }
    pub fn topic_conf(topic: &str) -> anyhow::Result<Self> {
        let topic_conf = ConfSub::topic_conf_str(topic);
        let confstr = get_or_empty(&topic_conf);
        let conf: ConfSub = serde_json::from_str(&confstr)?;
        Ok(conf)
    }
    pub fn save(&self) -> anyhow::Result<String> {
        let savemsg = serde_json::to_string(&self)?;
        let topic_conf_str = ConfSub::topic_conf_str(&self.topic);
        insert(&topic_conf_str, &savemsg);
        Ok(SUCCESS.to_owned())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SubPub {
    #[serde(default = "default_string")]
    pub topic: String,
    #[serde(default = "default_string")]
    pub answer: String,
    #[serde(default = "default_string")]
    pub time: String,
}

impl SubPub {
    pub fn pub_add(&mut self) -> anyhow::Result<String> {
        // 1个动作   把topic作为key存入消息队列，要把时间加上
        self.time = time_now_str();
        let savemsg = serde_json::to_string(&self)?;
        update_set_from_db_and_str(&self.topic, &savemsg);
        Ok(SUCCESS.to_owned())
    }
    pub fn db2vec(topic: &str) -> anyhow::Result<Vec<Self>> {
        let res = read_set_from_db(topic);
        let mut l = vec![];
        for s in res.iter() {
            let sp: SubPub = serde_json::from_str(&s)?;
            l.push(sp);
        }
        l.sort_by(|a, b| b.time.cmp(&a.time));
        Ok(l)
    }

    pub fn sub_get(topic: &str) -> anyhow::Result<String> {
        let data = SubPub::db2vec(topic)?;
        let mut res = vec![];
        for i in data.iter() {
            res.push(&i.answer);
        }
        let s = serde_json::to_string(&res)?;

        let conf = ConfSub::topic_conf(topic)?;
        if conf.consume == "once" {
            // 自消费一次，拿走就删除数据
            remove(topic);
        }
        return Ok(s);
    }
    pub fn clean_timeout_len(topic: &str) -> anyhow::Result<String> {
        let data = SubPub::db2vec(topic)?;
        // data倒排序
        let conf = ConfSub::topic_conf(topic)?;

        let mut newdata = HashSet::new();
        let max = conf.len as usize;
        for (i, s) in data.iter().enumerate() {
            // 超过长度
            if i > max {
                break;
            }
            // 超过时间
            let differ = time_differ(&s.time);
            if differ < conf.timeout {
                let ss = serde_json::to_string(&s)?;
                newdata.insert(ss);
            }
        }
        if newdata.len() == 0 {
            remove(topic);
        } else {
            let newstr = serde_json::to_string(&newdata)?;
            insert(topic, &newstr);
        }
        Ok(SUCCESS.to_owned())
    }
}

pub fn conf_sub_handler(_conf: web::Query<ConfSub>) -> anyhow::Result<String> {
    let conf = _conf.into_inner();
    let topic = &conf.topic;
    let topics: Topics = TopicManagement::default();
    // conf topic重复就失败；
    if topics.contains(topic) {
        let res = Status::feed_fail("duplicate topic conf");
        return res;
    }
    topics.add_topic(conf)?;
    Status::feed_ok()
}

pub fn publish_handler(mut subpub: SubPub) -> anyhow::Result<String> {
    // conf topic 没有就失败；
    let topic = subpub.topic.to_string();
    let topics: Topics = TopicManagement::default();
    if !topics.contains(&topic) {
        let res = Status::feed_fail("no topic in topic list");
        return res;
    }
    // 1个动作   把topic作为key存入消息队列，要把时间加上
    subpub.pub_add()?;
    // 在存放的地方根据数据是否过期删除数据
    Status::feed_ok()
}

pub fn subscribe_handler(topic: &str) -> anyhow::Result<String> {
    // conf topic 没有就失败；
    let topics: Topics = TopicManagement::default();
    if !topics.contains(topic) {
        let res = Status::feed_fail("no topic in topic list");
        return res;
    }
    // 在取数据的时候做清理
    SubPub::clean_timeout_len(topic)?;

    SubPub::sub_get(topic)
}
