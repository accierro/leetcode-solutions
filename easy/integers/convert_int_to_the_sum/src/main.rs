// Given an integer n. No-Zero integer is a positive integer which doesn't contain any 0 in its decimal representation.

// Return a list of two integers [A, B] where:

// A and B are No-Zero integers.
// A + B = n
// It's guarateed that there is at least one valid solution. If there are many valid solutions you can return any of them.

// Example 1:

// Input: n = 2
// Output: [1,1]
// Explanation: A = 1, B = 1. A + B = n and both A and B don't contain any 0 in their decimal representation.
// Example 2:

// Input: n = 11
// Output: [2,9]
// Example 3:

// Input: n = 10000
// Output: [1,9999]
// Example 4:

// Input: n = 69
// Output: [1,68]
// Example 5:

// Input: n = 1010
// Output: [11,999]

// Constraints:

// 2 <= n <= 10^4

struct Solution;

impl Solution {
    pub fn get_no_zero_integers(mut n: i32) -> Vec<i32> {
        let mut a = 0;
        let mut b = 0;
        let mut step = 1;

        while n > 0 {
            let d = n % 10;
            n /= 10;

            if (d == 0 || d == 1) && n > 0 {
                a += step * (1 + d);
                b += step * 9;
                n -= 1;
            } else {
                a += step * 1;
                b += step * (d - 1);
            }
            step *= 10;
        }

        vec![a, b]
    }
}

fn main() {
    assert_eq!(Solution::get_no_zero_integers(214), vec![1, 213]);
    assert_eq!(Solution::get_no_zero_integers(2), vec![1, 1]);
    assert_eq!(Solution::get_no_zero_integers(10000), vec![1, 9999]);
    assert_eq!(Solution::get_no_zero_integers(1057), vec![]);
    assert_eq!(Solution::get_no_zero_integers(10), vec![1, 9]);
    assert_eq!(Solution::get_no_zero_integers(11), vec![2, 9]);
    assert_eq!(Solution::get_no_zero_integers(104), vec![6, 99]);
}
