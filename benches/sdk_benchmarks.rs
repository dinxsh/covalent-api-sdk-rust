use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use goldrush_sdk::{
    GoldRushClient, ClientConfig, MetricsCollector, MemoryCache,
    Validator, Sanitizer, RateLimiter, RateLimitConfig,
};
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Runtime;

/// Benchmark validation operations.
fn bench_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("validation");
    
    let valid_address = "0x742d35Cc6634C0532925a3b8D4fc24f3C4aD6a8b";
    let valid_tx_hash = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
    
    group.bench_function("validate_address", |b| {
        b.iter(|| Validator::validate_address(valid_address))
    });
    
    group.bench_function("validate_tx_hash", |b| {
        b.iter(|| Validator::validate_tx_hash(valid_tx_hash))
    });
    
    group.bench_function("sanitize_address", |b| {
        b.iter(|| Sanitizer::sanitize_address("  0X742D35CC6634C0532925A3B8D4FC24F3C4AD6A8B  "))
    });
    
    group.finish();
}

/// Benchmark cache operations.
fn bench_cache(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("cache");
    
    let cache: MemoryCache<String> = MemoryCache::new(Duration::from_secs(60), 1000);
    
    group.bench_function("cache_set", |b| {
        b.to_async(&rt).iter(|| async {
            cache.set("test_key".to_string(), "test_value".to_string()).await;
        })
    });
    
    // Pre-populate cache for get benchmark
    rt.block_on(async {
        for i in 0..100 {
            cache.set(format!("key_{}", i), format!("value_{}", i)).await;
        }
    });
    
    group.bench_function("cache_get_hit", |b| {
        b.to_async(&rt).iter(|| async {
            cache.get("key_50").await
        })
    });
    
    group.bench_function("cache_get_miss", |b| {
        b.to_async(&rt).iter(|| async {
            cache.get("nonexistent_key").await
        })
    });
    
    group.finish();
}

/// Benchmark rate limiting operations.
fn bench_rate_limiting(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("rate_limiting");
    
    let rate_limiter = RateLimiter::new(RateLimitConfig {
        max_requests_per_second: 100.0,
        burst_capacity: 200,
        enable_backoff: true,
        max_retries: 3,
    });
    
    group.bench_function("rate_limit_acquire", |b| {
        b.to_async(&rt).iter(|| async {
            let _ = rate_limiter.acquire().await;
        })
    });
    
    group.finish();
}

/// Benchmark metrics collection.
fn bench_metrics(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("metrics");
    
    let metrics = MetricsCollector::new();
    
    group.bench_function("record_success", |b| {
        b.to_async(&rt).iter(|| async {
            metrics.record_success("test_endpoint", Duration::from_millis(100)).await;
        })
    });
    
    group.bench_function("record_error", |b| {
        b.to_async(&rt).iter(|| async {
            metrics.record_error("test_endpoint", "test_error").await;
        })
    });
    
    group.bench_function("get_metrics", |b| {
        b.to_async(&rt).iter(|| async {
            let _ = metrics.get_metrics().await;
        })
    });
    
    group.finish();
}

/// Benchmark client creation and configuration.
fn bench_client_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("client_operations");
    
    group.bench_function("client_creation", |b| {
        b.iter(|| {
            let config = ClientConfig::default();
            let _ = GoldRushClient::new("cqt_abcdefghij1234567890abcdef", config);
        })
    });
    
    group.bench_function("config_creation", |b| {
        b.iter(|| {
            let _ = ClientConfig::default();
        })
    });
    
    group.finish();
}

/// Benchmark serialization/deserialization performance.
fn bench_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialization");
    
    let sample_json = r#"{
        "data": {
            "items": [
                {
                    "contract_address": "0xa0b86991c431e59dce075d4c3ed7a68c8d96b8b4e",
                    "contract_name": "USD Coin",
                    "contract_decimals": 6,
                    "balance": "1000000",
                    "quote": 1000.0,
                    "quote_rate": 1.0
                }
            ],
            "pagination": {
                "has_more": false,
                "page_number": 0,
                "page_size": 100,
                "total_count": 1
            }
        }
    }"#;
    
    group.bench_function("json_parse", |b| {
        b.iter(|| {
            let _: serde_json::Value = serde_json::from_str(sample_json).unwrap();
        })
    });
    
    group.bench_function("json_stringify", |b| {
        let value: serde_json::Value = serde_json::from_str(sample_json).unwrap();
        b.iter(|| {
            let _ = serde_json::to_string(&value).unwrap();
        })
    });
    
    group.finish();
}

/// Benchmark memory usage patterns.
fn bench_memory_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_patterns");
    
    group.bench_function("string_allocation", |b| {
        b.iter(|| {
            let _strings: Vec<String> = (0..1000)
                .map(|i| format!("test_string_{}", i))
                .collect();
        })
    });
    
    group.bench_function("vector_growth", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for i in 0..1000 {
                vec.push(i);
            }
        })
    });
    
    group.finish();
}

/// Stress test for concurrent operations.
fn bench_concurrency(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("concurrency");
    
    let cache: Arc<MemoryCache<String>> = Arc::new(MemoryCache::new(Duration::from_secs(60), 10000));

    for threads in [1, 2, 4, 8, 16] {
        group.bench_with_input(
            BenchmarkId::new("concurrent_cache_operations", threads),
            &threads,
            |b, &threads| {
                b.to_async(&rt).iter(|| {
                    let cache = Arc::clone(&cache);
                    async move {
                        let handles: Vec<_> = (0..threads)
                            .map(|i| {
                                let cache = Arc::clone(&cache);
                                tokio::spawn(async move {
                                    for j in 0..10 {
                                        let key = format!("key_{}_{}", i, j);
                                        let value = format!("value_{}_{}", i, j);
                                        cache.set(key.clone(), value).await;
                                        let _ = cache.get(&key).await;
                                    }
                                })
                            })
                            .collect();

                        for handle in handles {
                            handle.await.unwrap();
                        }
                    }
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark different API endpoint patterns.
fn bench_endpoint_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("endpoint_patterns");
    
    let addresses = vec![
        "0x742d35Cc6634C0532925a3b8D4fc24f3C4aD6a8b",
        "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045",
        "0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5",
    ];
    
    group.bench_function("validate_multiple_addresses", |b| {
        b.iter(|| {
            for address in &addresses {
                let _ = Validator::validate_address(address);
            }
        })
    });
    
    group.bench_function("sanitize_multiple_addresses", |b| {
        b.iter(|| {
            for address in &addresses {
                let _ = Sanitizer::sanitize_address(address);
            }
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_validation,
    bench_cache,
    bench_rate_limiting,
    bench_metrics,
    bench_client_operations,
    bench_serialization,
    bench_memory_patterns,
    bench_concurrency,
    bench_endpoint_patterns
);

criterion_main!(benches);