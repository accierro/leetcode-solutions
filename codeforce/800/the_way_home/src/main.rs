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
    let mut scanner = Scanner::default();
    let info: Vec<usize> = (0..2).map(|_| scanner.next()).collect();
    let arr: Vec<char> = scanner.next::<String>().chars().collect();
    let mut dp = vec![false; arr.len()];
    dp[0] = true;
    dp[arr.len() - 1] = true;

    let mut count = 1;
    let mut current_pos = info[1];

    while !dp[std::cmp::min(current_pos, arr.len() - 1)] {
        dp[current_pos] = true;
        match arr[current_pos] {
            '1' => {
                current_pos += info[1];
                count += 1;
            }
            '0' => current_pos -= 1,
            _ => (),
        }
    }
    if current_pos >= arr.len() - 1 {
        println!("{}", count);
    } else {
        println!("-1");
    }
}
