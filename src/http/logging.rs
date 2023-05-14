use chrono::{SecondsFormat, Utc};
use std::time::Instant;

use super::middleware::*;
use crate::serializers::{to_json, Serialize};

#[derive(Default)]
pub struct LogMiddleware;

pub struct LogMiddlewareService<S> {
    service: S,
}

#[derive(Serialize, Default)]
struct Log {
    timestamp: String,
    method: String,
    path: String,
    query_params: String,
    http_version: String,
    client_ip: String,
    client_real_ip: String,
    request_headers: String,
    response_headers: String,
    status_code: String,
    latency: String,
}

impl Log {
    pub fn new(
        method: String,
        path: String,
        query_params: String,
        http_version: String,
        client_ip: String,
        client_real_ip: String,
    ) -> Self {
        Self {
            timestamp: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            method,
            path,
            query_params,
            http_version,
            client_ip,
            client_real_ip,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        to_json(self).unwrap().fmt(f)
    }
}

impl<S, B> Transform<S, ServiceRequest> for LogMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type InitError = ();
    type Transform = LogMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LogMiddlewareService { service }))
    }
}

impl<S, B> Service<ServiceRequest> for LogMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    #[inline]
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mut log = Log::new(
            req.method().to_string(),
            req.path().to_string(),
            req.query_string().to_string(),
            format!("{:?}", req.version()),
            req.connection_info()
                .peer_addr()
                .unwrap_or_default()
                .to_string(),
            req.connection_info()
                .realip_remote_addr()
                .unwrap_or_default()
                .to_string(),
        );

        let fut = self.service.call(req);

        Box::pin(async move {
            let start = Instant::now();
            let res = fut.await?;
            let elapsed = start.elapsed();

            log.request_headers = res
                .request()
                .headers()
                .iter()
                .fold(String::new(), |acc, (key, value)| {
                    format!("{}{}: {}\n", acc, key, value.to_str().unwrap())
                })
                .strip_suffix('\n')
                .unwrap_or_default()
                .to_string();
            log.response_headers = res
                .headers()
                .iter()
                .fold(String::new(), |acc, (key, value)| {
                    format!("{}{}: {}\n", acc, key, value.to_str().unwrap())
                })
                .strip_suffix('\n')
                .unwrap_or_default()
                .to_string();
            log.status_code = res.status().to_string();
            log.latency = format!("{:?}", elapsed);

            println!("{}", log);

            Ok(res)
        })
    }
}
