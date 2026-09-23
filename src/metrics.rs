use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

pub struct Metrics {
    pub total_requests: AtomicU64,
    pub success_requests: AtomicU64,
    pub client_errors: AtomicU64,
    pub server_errors: AtomicU64,
    pub network_errors: AtomicU64,
}

pub struct SnapshotMetrics {
    pub total_requests: u64,
    pub success_requests: u64,
    pub client_errors: u64,
    pub server_errors: u64,
    pub network_errors: u64,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            total_requests: AtomicU64::default(),
            success_requests: AtomicU64::default(),
            client_errors: AtomicU64::default(),
            server_errors: AtomicU64::default(),
            network_errors: AtomicU64::default(),
        }
    }

    pub fn record_response(&self, status_code: u16) {
        self.total_requests.fetch_add(1, Relaxed);
        match status_code {
            200..=299 => self.success_requests.fetch_add(1, Relaxed),
            400..=499 => self.client_errors.fetch_add(1, Relaxed),
            500..=599 => self.server_errors.fetch_add(1, Relaxed),
            _ => self.client_errors.fetch_add(1, Relaxed),
        };
    }
    pub fn record_failure(&self) {
        self.total_requests.fetch_add(1, Relaxed);
        self.network_errors.fetch_add(1, Relaxed);
    }

    pub fn snapshot(&self) -> SnapshotMetrics {
        SnapshotMetrics {
            total_requests: self.total_requests.load(Relaxed),
            success_requests: self.success_requests.load(Relaxed),
            client_errors: self.client_errors.load(Relaxed),
            server_errors: self.server_errors.load(Relaxed),
            network_errors: self.network_errors.load(Relaxed),
        }
    }
}
