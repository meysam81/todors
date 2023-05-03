use crate::serializers::{to_json, Serialize};
use chrono::{SecondsFormat, Utc};

#[derive(Serialize)]
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

impl Default for Log {
    fn default() -> Self {
        Self {
            timestamp: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            rpc: Default::default(),
            latency: Default::default(),
        }
    }
}
