//! 🔥 Бенчмарки для azlin-core
//! 
//! Запуск: cargo bench

use azlin_core::cache::{AzureCache, create_rg_cache};
use azlin_core::parallel::*;
use azlin_core::validation::VALID_AZURE_REGIONS;
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

/// Бенчмарк кэша
fn bench_cache(c: &mut Criterion) {
    let cache = AzureCache::new(60, 100);
    
    c.bench_function("cache_insert", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap()).iter(|| async {
            cache.insert(black_box("key".to_string()), black_box("value".to_string())).await;
        })
    });
    
    // Предварительно заполняем кэш
    futures::executor::block_on(async {
        for i in 0..100 {
            cache.insert(format!("key{}", i), format!("value{}", i)).await;
        }
    });
    
    c.bench_function("cache_get_hit", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap()).iter(|| async {
            cache.get_or_insert("key50", || async { Ok::<_, ()>("value".to_string()) }).await
        })
    });
    
    c.bench_function("cache_get_miss", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap()).iter(|| async {
            cache.get_or_insert("new_key", || async { Ok::<_, ()>("new_value".to_string()) }).await
        })
    });
}

/// Бенчмарк параллельной обработки
fn bench_parallel(c: &mut Criterion) {
    let data: Vec<i32> = (0..1000).collect();
    
    c.bench_function("par_cpu_process_1000", |b| {
        b.iter(|| {
            par_cpu_process(&data, |x| x * 2)
        })
    });
    
    let data_large: Vec<i32> = (0..10000).collect();
    
    c.bench_function("par_cpu_process_10000", |b| {
        b.iter(|| {
            par_cpu_process(&data_large, |x| x * 2)
        })
    });
    
    c.bench_function("fast_map_1000", |b| {
        b.iter(|| {
            fast_map(&data, |x| x * 2)
        })
    });
    
    c.bench_function("fast_filter_1000", |b| {
        b.iter(|| {
            fast_filter(&data, |x| x % 2 == 0)
        })
    });
}

/// Бенчмарк валидации
fn bench_validation(c: &mut Criterion) {
    c.bench_function("validate_region_eastus", |b| {
        b.iter(|| {
            azlin_core::validation::validate_azure_region(black_box("eastus"))
        })
    });
    
    c.bench_function("validate_region_westus2", |b| {
        b.iter(|| {
            azlin_core::validation::validate_azure_region(black_box("WestUS2"))
        })
    });
    
    // Бенчмарк для быстрого lookup
    use azlin_core::config_optimized::ConfigCache;
    let cache = ConfigCache::new();
    
    c.bench_function("cache_is_valid_region", |b| {
        b.iter(|| {
            cache.is_valid_region(black_box("eastus"))
        })
    });
    
    c.bench_function("cache_canonicalize_region", |b| {
        b.iter(|| {
            cache.canonicalize_region(black_box("WestUS2"))
        })
    });
}

/// Бенчмарк для разных размеров данных
fn bench_data_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_processing");
    
    for size in [100, 1000, 10000].iter() {
        let data: Vec<i32> = (0..*size).collect();
        
        group.bench_with_input(BenchmarkId::new("par_cpu_process", size), &data, |b, data| {
            b.iter(|| par_cpu_process(data, |x| x * 2))
        });
        
        group.bench_with_input(BenchmarkId::new("fast_map", size), &data, |b, data| {
            b.iter(|| fast_map(data, |x| x * 2))
        });
    }
    
    group.finish();
}

/// Бенчмарк группировки
fn bench_grouping(c: &mut Criterion) {
    let data: Vec<String> = (0..1000).map(|i| format!("item_{}", i % 10)).collect();
    
    c.bench_function("par_group_by_1000", |b| {
        b.iter(|| {
            par_group_by(data.clone(), |s| s.clone())
        })
    });
    
    let data_reduce: Vec<i32> = (0..1000).collect();
    
    c.bench_function("par_reduce_sum_1000", |b| {
        b.iter(|| {
            par_reduce(&data_reduce, 0, |a, b| a + b)
        })
    });
}

/// Бенчмарк batch обработки
fn bench_batch(c: &mut Criterion) {
    let data: Vec<i32> = (0..1000).collect();
    
    c.bench_function("batch_process_100", |b| {
        b.iter(|| {
            batch_process(&data, 100, |batch| batch.iter().sum::<i32>())
        })
    });
    
    c.bench_function("batch_process_50", |b| {
        b.iter(|| {
            batch_process(&data, 50, |batch| batch.iter().sum::<i32>())
        })
    });
}

criterion_group!(
    benches,
    bench_cache,
    bench_parallel,
    bench_validation,
    bench_data_sizes,
    bench_grouping,
    bench_batch,
);

criterion_main!(benches);
