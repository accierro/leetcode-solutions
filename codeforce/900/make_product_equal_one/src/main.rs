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

    let n = scan.next::<i32>();
    let arr: Vec<i64> = (0..n).map(|_| scan.next()).collect();

    let mut zeroes = 0;
    let mut negative = 0;
    let mut res = 0;

    for i in arr.iter() {
        if *i == 0 {
            zeroes += 1;
        } else {
            res += (*i).abs() - 1;
        }

        if *i < 0 {
            negative += 1;
        }
    }
    res += zeroes;
    if negative % 2 == 1 && zeroes == 0 {
        res += 2;
    }

    println!("{}", res)
}
