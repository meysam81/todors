use std::collections::HashMap;

use crate::consts;

use actix_web_prometheus::{PrometheusMetrics, PrometheusMetricsBuilder};

pub fn build_metrics() -> PrometheusMetrics {
    let mut labels = HashMap::new();
    labels.insert("app".to_string(), "todors".to_string());

    PrometheusMetricsBuilder::new(consts::DEFAULT_METRICS_NAMESPACE)
        .endpoint(consts::DEFAULT_METRICS_URI)
        .build()
        .unwrap()
}

#[allow(dead_code)]
#[utoipa::path(
        get,
        path = "/metrics",
        operation_id = "Prometheus metrics",
        responses(
            (status = 200, content_type = "text/plain", body = String),
        ),
        tag = "metrics",
    )]
pub fn metrics_api() {}
