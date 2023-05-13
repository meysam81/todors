use actix_web_prometheus::{PrometheusMetrics, PrometheusMetricsBuilder};

pub fn build_metrics() -> PrometheusMetrics {
    PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
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
