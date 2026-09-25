use crate::cli::Cli;
use crate::metrics::Metrics;
use bytes::Bytes;
use reqwest::{
    Client, Url,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

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
            if let Some((key, val)) = h.split_once(':') {
                if let Ok(name) = HeaderName::from_bytes(key.trim().as_bytes()) {
                    if let Ok(value) = HeaderValue::from_str(val.trim()) {
                        header_map.insert(name, value);
                    }
                }
            }
        }

        let payload: Option<Bytes> = self.args.body.as_ref().map(|b| Bytes::from(b.clone()));
        let target_url: Url = self.args.url.parse().expect("invalid url");
        let client = Client::builder()
            .default_headers(header_map)
            .tcp_nodelay(true)
            .user_agent("pulse/0.1.0")
            .pool_max_idle_per_host(self.args.concurrency)
            .timeout(Duration::from_secs(5))
            .build()
            .expect("Failed to build client");

        let start = Instant::now();

        let stop_signal = Arc::new(AtomicBool::new(false));
        let mut join_set = JoinSet::new();

        println!("Running test on  {}", self.args.duration.as_secs());

        for _ in 0..self.args.concurrency {
            let client = client.clone();
            let metrics = Arc::clone(&self.metrics);
            let payload = payload.clone();
            let stop = Arc::clone(&stop_signal);
            let url = target_url.clone();

            join_set.spawn(async move {
                while !stop.load(Ordering::Relaxed) {
                    let mut req = client.post(url.clone());
                    if let Some(ref body) = payload {
                        req = req.body(body.clone());
                    }

                    match req.send().await {
                        Ok(resp) => {
                            metrics.record_response(resp.status().as_u16());
                            let _ = resp.bytes().await;
                        }
                        Err(_) => {
                            metrics.record_failure();
                        }
                    }
                }
            });
        }

        tokio::time::sleep(self.args.duration).await;
        stop_signal.store(true, Ordering::Relaxed);

        while join_set.join_next().await.is_some() {}

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
