//! ⚡ Высокопроизводительные параллельные операции
//! 
//! Использует rayon для CPU-bound задач и tokio для I/O-bound.

use rayon::prelude::*;
use std::future::Future;
use tokio::task::JoinSet;

/// Параллельная обработка коллекции с ограничением параллелизма
/// 
/// # Примеры
/// 
/// ```rust,ignore
/// let vms = vec![vm1, vm2, vm3];
/// let results = par_process_with_limit(vms, 10, |vm| async {
///     process_vm(vm).await
/// }).await;
/// ```
pub async fn par_process_with_limit<T, F, Fut, R>(
    items: Vec<T>,
    limit: usize,
    processor: F,
) -> Vec<R>
where
    T: Send + 'static,
    R: Send + 'static,
    F: Fn(T) -> Fut + Send + Sync + Clone + 'static,
    Fut: Future<Output = R> + Send + 'static,
{
    let mut set = JoinSet::new();
    let mut results = Vec::with_capacity(items.len());
    let mut pending = Vec::new();

    for item in items {
        while set.len() >= limit {
            if let Some(res) = set.join_next().await {
                results.push(res.unwrap());
            }
        }
        
        let proc = processor.clone();
        set.spawn(async move { proc(item).await });
    }

    // Ждем оставшиеся задачи
    while let Some(res) = set.join_next().await {
        results.push(res.unwrap());
    }

    results
}

/// Параллельная CPU-bound обработка с использованием rayon
/// 
/// # Примеры
/// 
/// ```rust
/// let data = vec![1, 2, 3, 4, 5];
/// let results = par_cpu_process(&data, |x| x * 2);
/// assert_eq!(results, vec![2, 4, 6, 8, 10]);
/// ```
pub fn par_cpu_process<T, R, F>(data: &[T], processor: F) -> Vec<R>
where
    T: Send + Sync,
    R: Send + 'static,
    F: Fn(&T) -> R + Send + Sync,
{
    data.par_iter().map(&processor).collect()
}

/// Пакетная обработка с группировкой по N элементов
/// 
/// # Примеры
/// 
/// ```rust
/// let items: Vec<i32> = (1..=10).collect();
/// let batches = batch_process(&items, 3, |batch| batch.iter().sum::<i32>());
/// assert_eq!(batches, vec![6, 15, 24, 10]); // 1+2+3, 4+5+6, 7+8+9, 10
/// ```
pub fn batch_process<T, R, F>(items: &[T], batch_size: usize, processor: F) -> Vec<R>
where
    F: Fn(&[T]) -> R,
{
    items
        .chunks(batch_size)
        .map(&processor)
        .collect()
}

/// Асинхронная пакетная обработка с ограничением
pub async fn par_batch_async<T, F, Fut, R>(
    items: Vec<T>,
    batch_size: usize,
    concurrency: usize,
    processor: F,
) -> Vec<R>
where
    T: Send + Clone + 'static,
    R: Send + 'static,
    F: Fn(Vec<T>) -> Fut + Send + Sync + Clone + 'static,
    Fut: Future<Output = R> + Send + 'static,
{
    let batches: Vec<Vec<T>> = items
        .chunks(batch_size)
        .map(|chunk| chunk.to_vec())
        .collect();

    par_process_with_limit(batches, concurrency, processor).await
}

/// Быстрая фильтрация с использованием SIMD (где доступно)
#[inline]
pub fn fast_filter<T, F>(data: &[T], predicate: F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> bool + Send + Sync,
{
    data.par_iter()
        .filter(|item| predicate(item))
        .cloned()
        .collect()
}

/// Оптимизированный map с предварительным выделением памяти
#[inline]
pub fn fast_map<T, R, F>(data: &[T], processor: F) -> Vec<R>
where
    F: Fn(&T) -> R,
{
    let mut result = Vec::with_capacity(data.len());
    for item in data {
        result.push(processor(item));
    }
    result
}

/// Группировка по ключу с параллельной обработкой
pub fn par_group_by<T, K, F>(items: Vec<T>, key_extractor: F) -> std::collections::HashMap<K, Vec<T>>
where
    K: std::hash::Hash + Eq + Send + Sync + 'static,
    T: Send + 'static,
    F: Fn(&T) -> K + Send + Sync + Clone + 'static,
{
    use std::collections::HashMap;
    use dashmap::DashMap;

    let map = DashMap::new();

    items.into_par_iter().for_each(|item| {
        let key = key_extractor(&item);
        map.entry(key).or_insert_with(Vec::new).push(item);
    });

    map.into_iter().collect()
}

/// Reduce операция с параллельной обработкой
pub fn par_reduce<T, F>(data: &[T], identity: T, reducer: F) -> T
where
    T: Send + Sync + Clone,
    F: Fn(T, T) -> T + Send + Sync,
{
    data.par_iter()
        .fold(
            || identity.clone(),
            |acc, item| reducer(acc, item.clone()),
        )
        .reduce(|| identity, |a, b| reducer(a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_par_cpu_process() {
        let data = vec![1, 2, 3, 4, 5];
        let results = par_cpu_process(&data, |x| x * 2);
        assert_eq!(results, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_batch_process() {
        let items: Vec<i32> = (1..=10).collect();
        let batches = batch_process(&items, 3, |batch| batch.iter().sum::<i32>());
        assert_eq!(batches, vec![6, 15, 24, 10]);
    }

    #[test]
    fn test_fast_filter() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let filtered = fast_filter(&data, |x| x % 2 == 0);
        assert_eq!(filtered, vec![2, 4, 6]);
    }

    #[test]
    fn test_fast_map() {
        let data = vec![1, 2, 3];
        let mapped = fast_map(&data, |x| x * 2);
        assert_eq!(mapped, vec![2, 4, 6]);
    }

    #[tokio::test]
    async fn test_par_process_with_limit() {
        let items = vec![1, 2, 3, 4, 5];
        let results = par_process_with_limit(items, 2, |x| async move { x * 2 }).await;
        assert_eq!(results.len(), 5);
        assert!(results.contains(&2));
        assert!(results.contains(&10));
    }

    #[test]
    fn test_par_group_by() {
        let items = vec!["a", "bb", "ccc", "dd", "e"];
        let grouped = par_group_by(items, |s| s.len());
        
        assert_eq!(grouped.get(&1).unwrap().len(), 2); // "a", "e"
        assert_eq!(grouped.get(&2).unwrap().len(), 2); // "bb", "dd"
        assert_eq!(grouped.get(&3).unwrap().len(), 1); // "ccc"
    }

    #[test]
    fn test_par_reduce() {
        let data = vec![1, 2, 3, 4, 5];
        let sum = par_reduce(&data, 0, |a, b| a + b);
        assert_eq!(sum, 15);
    }
}
