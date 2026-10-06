//! # Tokio Async Tasks
//!
//! In this exercise, you will use `tokio::spawn` to create concurrent asynchronous tasks.
//!
//! ## Concepts
//! - `tokio::spawn` creates asynchronous tasks
//! - `JoinHandle` waits for task completion
//! - Concurrent execution between asynchronous tasks

/*
1.async {}, 异步块是一个表达式，编译器会根据这个，生成一个实现了Future trait的谜名数据类型的实例
他可以捕获外部的数据，捕获的数据都会写在幂名结构体里面，其中的关联数据类型output，由异步块的最后返回的数据类型进行推导，自动实现poll的函数，轮询逻辑采用编译器生成的默认逻辑
2.在异步块里面写需要执行的函数逻辑,异步块最后返回的值就是整体返回的值，tokio::spawn()函数，将异步块作为参数，把它交给后台运行，返回handle句柄
3.handle.await 会根据future对象，根据里面的poll轮询逻辑，多次访问直到，运行完毕，返回Result<Future::output,err>
 */


use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

/// Concurrently compute the square of each number in 0..n, collect results and return in order.
///
/// Hint: Create `tokio::spawn` task for each i, collect JoinHandle, await them sequentially.
pub async fn concurrent_squares(n: usize) -> Vec<usize> {
    // TODO: Create n asynchronous tasks, each computing i * i
    // TODO: Collect all JoinHandle
    // TODO: Await each one to get result
    let mut result = Vec::new();
    let mut handles = Vec::new();
    for i in 0..n {
        handles.push(tokio::spawn(async move{
            i * i
        }));
    }

    for h in handles {
        result.push(h.await.unwrap());
    }

    result
}

/// Concurrently execute multiple "time-consuming" tasks (simulated with sleep), return all results.
/// Each task sleeps `duration_ms` milliseconds and then returns its `task_id`.
///
/// Key: All tasks should execute concurrently, total duration should be close to single task duration, not sum of all tasks.
pub async fn parallel_sleep_tasks(n: usize, duration_ms: u64) -> Vec<usize> {
    // TODO: Create asynchronous task for each id in 0..n
    // TODO: Each task sleeps specified duration and returns its own id
    // TODO: Collect all results and sort
    let mut result = Vec::new();
    let mut handles = Vec::new();
    for task_id in 0..n {
        handles.push(tokio::spawn(async move {
            sleep(Duration::from_millis(duration_ms));
            task_id
        }));
    }

    for h in handles {
        result.push(h.await.unwrap());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Instant;

    #[tokio::test]
    async fn test_squares_basic() {
        let result = concurrent_squares(5).await;
        assert_eq!(result, vec![0, 1, 4, 9, 16]);
    }

    #[tokio::test]
    async fn test_squares_zero() {
        let result = concurrent_squares(0).await;
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_squares_one() {
        let result = concurrent_squares(1).await;
        assert_eq!(result, vec![0]);
    }

    #[tokio::test]
    async fn test_parallel_sleep() {
        let start = Instant::now();
        let result = parallel_sleep_tasks(5, 100).await;
        let elapsed = start.elapsed();

        assert_eq!(result, vec![0, 1, 2, 3, 4]);
        // Concurrent execution, total time should be much less than 5 * 100ms
        assert!(
            elapsed.as_millis() < 400,
            "Tasks should run concurrently, took {}ms",
            elapsed.as_millis()
        );
    }
}
