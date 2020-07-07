use std::cmp::{max, min};
use std::io::{stdin, stdout, BufWriter, Write};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() {
    let mut scan = Scanner::default();
    let n = scan.next::<usize>();
    let arr: Vec<usize> = (0..n).map(|_| scan.next()).collect();

    let mut dp = vec![0; arr.len()];
    dp[0] = 1;
    let mut max = 1;

    for i in 1..arr.len() {
        if arr[i] > arr[i - 1] {
            dp[i] = dp[i - 1] + 1;
            max = std::cmp::max(dp[i], max);
        } else {
            dp[i] = 1;
        }
    }

    println!("{}", max);
}
