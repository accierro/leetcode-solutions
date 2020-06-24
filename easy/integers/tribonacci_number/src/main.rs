// The Tribonacci sequence Tn is defined as follows:

// T0 = 0, T1 = 1, T2 = 1, and Tn+3 = Tn + Tn+1 + Tn+2 for n >= 0.

// Given n, return the value of Tn.

// Example 1:

// Input: n = 4
// Output: 4
// Explanation:
// T_3 = 0 + 1 + 1 = 2
// T_4 = 1 + 1 + 2 = 4
// Example 2:

// Input: n = 25
// Output: 1389537

// Constraints:

// 0 <= n <= 37
// The answer is guaranteed to fit within a 32-bit integer, ie. answer <= 2^31 - 1.

struct Solution;

impl Solution {
    pub fn tribonacci(mut n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 1;
        }
        let mut n0 = 0;
        let mut n1 = 1;
        let mut n2 = 1;

        while n > 3 {
            let temp = n0 + n1 + n2;
            n0 = n1;
            n1 = n2;
            n2 = temp;
            n -= 1;
        }

        n0 + n1 + n2
    }
}

fn main() {
    assert_eq!(Solution::tribonacci(3), 2);
    assert_eq!(Solution::tribonacci(25), 1389537);
    assert_eq!(Solution::tribonacci(4), 4);
}
