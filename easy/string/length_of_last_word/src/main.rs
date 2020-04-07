// Given a string s consists of upper/lower-case alphabets and empty space characters ' ', return the length of last word (last word means the last appearing word if we loop from left to right) in the string.

// If the last word does not exist, return 0.

// Note: A word is defined as a maximal substring consisting of non-space characters only.

// Example:

// Input: "Hello World"
// Output: 5

struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let trimmed_str = s.trim_end().to_string();
        for (i, ch) in trimmed_str.chars().rev().enumerate() {
            if ch == ' ' {
                return i as i32;
            } else if i == trimmed_str.len() - 1 {
                return (i + 1) as i32;
            }
        }
        0
    }
}

fn main() {
    assert_eq!(
        Solution::length_of_last_word(String::from("Hello world")),
        5
    );
    assert_eq!(Solution::length_of_last_word(String::from("      ")), 0);
    assert_eq!(Solution::length_of_last_word(String::from("a")), 1);
}
