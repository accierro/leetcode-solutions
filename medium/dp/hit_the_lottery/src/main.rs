fn solution(mut n: i32) -> i32 {
    let mut count = 0;
    let bills = vec![1, 5, 10, 20, 100];

    for i in bills.iter().rev() {
        let div = n / i;
        count += div;
        if div != 0 {
            n %= div * i;
        }
    }

    count
}

fn main() {
    assert_eq!(solution(125), 3);
    assert_eq!(solution(43), 5);
    assert_eq!(solution(1000000000), 10000000);
}
