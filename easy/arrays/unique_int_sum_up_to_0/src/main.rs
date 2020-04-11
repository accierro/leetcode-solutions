// Given an integer n, return any array containing n unique integers such that they add up to 0.

// Example 1:

// Input: n = 5
// Output: [-7,-1,1,3,4]
// Explanation: These arrays also are accepted [-5,-1,1,2,3] , [-3,-1,2,-2,4].
// Example 2:

// Input: n = 3
// Output: [-1,0,1]
// Example 3:

// Input: n = 1
// Output: [0]

// Constraints:

// 1 <= n <= 1000

struct Solution;

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        if n % 2 == 1 {
            result.push(0);
        }
        for i in 1..n / 2 + 1 {
            result.push(i);
            result.push(i * -1);
        }
        result
    }
}

fn main() {
    assert_eq!(Solution::sum_zero(5).iter().sum::<i32>(), 0);
    assert_eq!(Solution::sum_zero(4).iter().sum::<i32>(), 0);
}
