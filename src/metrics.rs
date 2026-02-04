#[cfg(feature = "std")]
use std::sync::atomic::{AtomicU64, Ordering};
#[cfg(feature = "std")]
use std::sync::OnceLock;

#[cfg(feature = "std")]
static METRICS_ENABLED: OnceLock<bool> = OnceLock::new();
#[cfg(feature = "std")]
static COMMIT_COUNT: AtomicU64 = AtomicU64::new(0);
#[cfg(feature = "std")]
static HASH_COUNT: AtomicU64 = AtomicU64::new(0);

#[cfg(feature = "std")]
fn metrics_enabled() -> bool {
    *METRICS_ENABLED.get_or_init(|| match std::env::var("BONSAI_HASH_METRICS") {
        Ok(value) => value != "0",
        Err(_) => false,
    })
}

#[cfg(feature = "std")]
pub(crate) fn record_hashes(count: usize, identifier: &[u8]) {
    if !metrics_enabled() {
        return;
    }

    let count = count as u64;
    let commits = COMMIT_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    let total = HASH_COUNT.fetch_add(count, Ordering::Relaxed) + count;
    let avg = total as f64 / commits as f64;

    eprintln!(
        "bonsai-hash-metrics: commit={} identifier_len={} hashes={} avg_hashes={:.2}",
        commits,
        identifier.len(),
        count,
        avg
    );
}
