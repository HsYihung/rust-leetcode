use std::time::{Duration, Instant};
use std::mem;

pub struct Metrics {
    pub execution_time: Duration,
    pub memory_size: usize,
}

pub fn measure_time_and_space<F, T>(f: F) -> (T, Metrics)
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();

    
    let size = mem::size_of_val(&result);

    (result, Metrics {
        execution_time: duration,
        memory_size: size,
    })
}

pub fn generate_test_data(size: usize) -> Vec<i32> {
    (0..size as i32).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measure_performance() {
        let (result, metrics) = measure_time_and_space(|| {
            let mut sum = 0;
            for i in 0..1000 {
                sum += i;
            }
            sum
        });

        println!("Result: {}", result);
        println!("Execution time: {:?}", metrics.execution_time);
        println!("Memory size: {} bytes", metrics.memory_size);
    }
}
