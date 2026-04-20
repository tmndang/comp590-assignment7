fn main() {
    assert_eq!(collatz_length(1), 1, "length of 1 should be 1");
    assert_eq!(collatz_length(6), 9, "length of 6 should be 9");
    assert_eq!(collatz_length(27), 112, "length of 27 should be 112");
    println!("collatz_length checks passed.");

    let the_answer = longest_collatz(1_000_000);
    assert_eq!(the_answer, 837799, "longest below 1M should be 837799");
    println!("longest_collatz(1_000_000) = {}", the_answer);
    println!("All checks passed.");
}

fn collatz_length(n: u64) -> u64 {
    fn helper(n: u64, acc: u64) -> u64 {
        match n {
            1 => acc,
            n if n % 2 == 0 => helper(n / 2, acc + 1),
            _ => helper(3 * n + 1, acc + 1),
        }
    }
    helper(n, 1)
}

fn longest_collatz(limit: u64) -> u64 {
    fn helper(current: u64, limit: u64, best_start: u64, best_len: u64) -> u64 {
        if current >= limit { return best_start; }
        let len = collatz_length(current);
        if len > best_len {
            helper(current + 1, limit, current, len)
        } else {
            helper(current + 1, limit, best_start, best_len)
        }
    }
    std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(move || helper(1, limit, 1, 1))
        .unwrap()
        .join()
        .unwrap()
}