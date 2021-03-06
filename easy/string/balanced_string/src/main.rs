// Balanced strings are those who have equal quantity of 'L' and 'R' characters.

// Given a balanced string s split it in the maximum amount of balanced strings.

// Return the maximum amount of splitted balanced strings.

// Example 1:

// Input: s = "RLRRLLRLRL"
// Output: 4
// Explanation: s can be split into "RL", "RRLL", "RL", "RL", each substring contains same number of 'L' and 'R'.
// Example 2:

// Input: s = "RLLLLRRRLR"
// Output: 3
// Explanation: s can be split into "RL", "LLLRRR", "LR", each substring contains same number of 'L' and 'R'.
// Example 3:

// Input: s = "LLLLRRRR"
// Output: 1
// Explanation: s can be split into "LLLLRRRR".
// Example 4:

// Input: s = "RLRRRLLRLL"
// Output: 2
// Explanation: s can be split into "RL", "RRRLLRLL", since each substring contains an equal number of 'L' and 'R'

// Constraints:

// 1 <= s.length <= 1000
// s[i] = 'L' or 'R'

struct Solution;

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut balance = 0;
        let mut result = 0;

        for ch in s.chars() {
            match ch {
                'R' => balance -= 1,
                'L' => balance += 1,
                _ => panic!("Incorrect char provided"),
            }
            if balance == 0 {
                result += 1;
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::balanced_string_split(String::from("RLRRLLRLRL")),
        4
    );
    assert_eq!(
        Solution::balanced_string_split(String::from("RLLLLRRRLR")),
        3
    );
    assert_eq!(Solution::balanced_string_split(String::from("LLLLRRRR")), 1);
    assert_eq!(
        Solution::balanced_string_split(String::from("RLRRRLLRLL")),
        2
    );
}
