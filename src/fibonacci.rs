use nexus_rt::write_log;

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

pub fn run_fibonacci(n: u32, expected_result: u32) {
    let result = fib(n);
    assert_eq!(result, expected_result);
    
    write_log("Fibonacci ran successfully!\n");
}