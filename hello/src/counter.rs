use serde::{Deserialize, Serialize};
use crate::config::Config;

#[derive(Serialize, Deserialize, Debug)]
pub struct Counter {
    pub count: i32
}

impl Counter {
    fn get_redis_url() -> String {
        let config = Config::new();
        config.map.get("REDIS_URL")
            .unwrap().as_str()
            .unwrap().to_owned()
    }

    pub fn save(self) {

    }

    pub fn load() -> Counter {

    }
} 