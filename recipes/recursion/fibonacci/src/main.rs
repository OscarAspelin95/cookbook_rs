/// Here, we define fib(0) as the first number (zero-indexed).
fn fib(n: usize) -> usize {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    fib(n - 1) + fib(n - 2)
}

fn main() {
    assert_eq!(fib(0), 0);
    assert_eq!(fib(1), 1);
    assert_eq!(fib(2), 1);
    assert_eq!(fib(3), 2);
    assert_eq!(fib(9), 34);
}
