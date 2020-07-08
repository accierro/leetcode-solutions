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
    let arr: Vec<u32> = (0..n).map(|_| scan.next()).collect();

    let mut dp = vec![1; arr.len()];
    let mut res = dp[0];

    for i in 1..arr.len() {
        if arr[i] >= arr[i - 1] {
            dp[i] = dp[i - 1] + 1;

            if dp[i] > res {
                res = dp[i];
            }
        }
    }

    println!("{}", res);
}
