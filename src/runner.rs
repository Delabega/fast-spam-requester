use bytes::Bytes;
use reqwest::{
    Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;

use crate::cli::Cli;
use crate::metrics::Metrics;

pub struct Runner {
    args: Cli,
    metrics: Arc<Metrics>,
}

impl Runner {
    pub fn new(args: Cli) -> Self {
        Self {
            args,
            metrics: Arc::new(Metrics::new()),
        }
    }

    pub async fn run(&self) {
        let mut header_map = HeaderMap::new();

        for h in &self.args.headers {
            if let Some((key, val)) = h.split_once(':')
                && let Ok(name) = HeaderName::from_bytes(key.trim().as_bytes())
                && let Ok(value) = HeaderValue::from_str(val.trim())
            {
                header_map.insert(name, value);
            }
        }

        let payload: Option<Bytes> = self.args.body.as_ref().map(|b| Bytes::from(b.clone()));
        let client = Client::builder()
            .default_headers(header_map)
            .tcp_nodelay(true)
            .user_agent("pulse/0.1.0")
            .pool_max_idle_per_host(self.args.concurrency)
            .timeout(Duration::from_secs(5))
            .build()
            .expect("Failed to build client");
        let semaphore = Arc::new(Semaphore::new(self.args.concurrency));

        let start = Instant::now();
        let target_url = self.args.url.clone();

        println!("Running test on  {}", self.args.duration.as_secs());

        tokio::select! {
        _ = tokio::time::sleep(self.args.duration) => {
            println!("Time`s up");
        }
        _ = async {
            loop {
                let permit = match Arc::clone(&semaphore).acquire_owned().await {
                    Ok(p) => p,
                    Err(_) => break,
                };

                let client = client.clone();
                let url = target_url.clone();
                let metrics = Arc::clone(&self.metrics);
                let payload = payload.clone();

                tokio::spawn(async move {
                    let mut req = client.post(&url);
                    if let Some(body) = payload {
                        req = req.body(body);
                    }

                    match req.send().await {
                        Ok(resp) => {
                            metrics.record_response(resp.status().as_u16());
                        }
                        Err(_) => {
                             metrics.record_failure();
                        }
                    }

                    drop(permit);
                });
                    }
                } => {}
            }
        let _ = semaphore.acquire_many(self.args.concurrency as u32).await;

        let total_time = start.elapsed().as_secs_f64();
        let stats = self.metrics.snapshot();

        let rps = if total_time > 0.0 {
            stats.total_requests as f64 / total_time
        } else {
            0.0
        };

        println!("Duration:           {:.2}s", total_time);
        println!("Requests per sec:   {:.2}", rps);
        println!("Total requests:     {}", stats.total_requests);
        println!("Successful (2xx):   {}", stats.success_requests);
        println!("Client errors (4xx):{}", stats.client_errors);
        println!("Server errors (5xx):{}", stats.server_errors);
        println!("Network failures:   {}", stats.network_errors);
    }
}
