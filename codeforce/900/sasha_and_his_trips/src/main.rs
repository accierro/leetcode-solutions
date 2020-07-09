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

    let info: Vec<i32> = (0..2).map(|_| scan.next()).collect();
    let mut cost = 0;
    let mut tank = 0;

    if info[0] - 1 <= info[1] {
        println!("{}", info[0] - 1);
        return;
    }

    for i in 0..info[0] - 1 {
        if tank < (info[0] - 1 - i) {
            cost += (info[1] - tank) * (i + 1);
            tank = info[1];
        }
        tank -= 1;
    }

    println!("{}", cost);
}
