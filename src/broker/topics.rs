use crate::broker::subpub::ConfSub;
use crate::common::sled_db::*;
use crate::common::sled_db::{insert, read_set_from_db, update_set_from_db_and_str};
use crate::common::tools::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub type Topics = HashSet<String>;

pub trait TopicManagement {
    fn default() -> Self;
    fn topic_exist(&self, topic: &str) -> bool;
    fn add_topic(&self, conf: ConfSub) -> anyhow::Result<String>;
    fn del_topic(&self, topic: &str) -> anyhow::Result<String>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub status: String,
    // timeout,length,takeout times
    pub reason: String,
}

impl Status {
    pub fn default() -> Self {
        Status {
            status: SUCCESS.to_string(),
            reason: "".to_string(),
        }
    }
    pub fn feed_ok() -> anyhow::Result<String> {
        let feedback = Status::default();
        let res = serde_json::to_string(&feedback)?;
        Ok(res)
    }
    pub fn feed_fail(reason: &str) -> anyhow::Result<String> {
        let mut feedback = Status::default();
        feedback.status = FAIL.to_string();
        feedback.reason = reason.to_string();
        let res = serde_json::to_string(&feedback)?;
        Ok(res)
    }
}

impl TopicManagement for Topics {
    fn default() -> Self {
        read_set_from_db(DB_TOPICS)
    }
    fn topic_exist(&self, topic: &str) -> bool {
        self.contains(topic)
    }
    fn add_topic(&self, conf: ConfSub) -> anyhow::Result<String> {
        // 两个动作 1 把topic放到set 2 把topic——conf作为key存入
        let topic = conf.topic.clone();
        update_set_from_db_and_str(DB_TOPICS, &topic);

        conf.save()?;
        Ok(SUCCESS.to_owned())
    }
    fn del_topic(&self, topic: &str) -> anyhow::Result<String> {
        let mut newset = HashSet::new();
        for i in self.iter() {
            if i != topic {
                newset.insert(i);
            }
        }
        let newres = serde_json::to_string(&newset)?;

        // 从topics删除，并且删除name——conf，以及name
        insert(DB_TOPICS, &newres);
        remove(topic);

        let mut topic_conf = topic.clone().to_owned();
        topic_conf.push_str(SUFFIX_CONF);
        remove(&topic_conf);
        Ok(SUCCESS.to_owned())
    }
}

pub fn get_topics_handler() -> anyhow::Result<String> {
    let res = get_or_empty(DB_TOPICS);
    Ok(res)
}

pub fn del_topics_handler(topic: &str) -> anyhow::Result<String> {
    let existed = read_set_from_db(DB_TOPICS);
    let mut newset = HashSet::new();
    for i in existed.iter() {
        if i != topic {
            newset.insert(i);
        }
    }
    let newres = serde_json::to_string(&newset)?;

    // 从topics删除，并且删除name——conf，以及name
    insert(DB_TOPICS, &newres);
    remove(topic);

    let mut topic_conf = topic.clone().to_owned();
    topic_conf.push_str(SUFFIX_CONF);
    remove(&topic_conf);

    let feedback = Status::default();
    let res = serde_json::to_string(&feedback)?;
    Ok(res)
}
