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
    let mut count = 0;

    let mut scanner = Scanner::default();
    let s = scanner.next::<String>();

    let start = std::time::Instant::now();
    let s_chars = s.chars().collect::<Vec<char>>();

    let mut first_q = vec![0; s_chars.len()];
    let mut a = vec![];

    first_q[0] = if s_chars[0] == 'Q' { 1 } else { 0 };

    for i in 1..s_chars.len() {
        first_q[i] = first_q[i - 1];
        match s_chars[i] {
            'Q' => {
                first_q[i] += 1;
            }
            'A' => a.push(i),
            _ => (),
        }
    }

    for i in a {
        count += first_q[i] * (first_q.last().unwrap() - first_q[i]);
    }

    println!("{}", count);
    println!("{:?}", start.elapsed());
}
