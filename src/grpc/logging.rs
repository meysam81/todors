use crate::serializers::{to_json, Serialize};
use chrono::{SecondsFormat, Utc};

#[derive(Serialize, Default)]
pub struct Log {
    pub timestamp: String,
    pub rpc: String,
    pub latency: String,
}

impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        to_json(self).unwrap().fmt(f)
    }
}

impl Log {
    pub fn new(rpc: &str) -> Self {
        Self {
            timestamp: Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
            rpc: rpc.to_string(),
            ..Default::default()
        }
    }
}
