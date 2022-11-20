// 递归
pub fn fib_pre(n: usize) -> usize {
    if n < 2 {
        1
    } else {
        fib_pre(n - 1) + fib_pre(n - 2)
    }
}
