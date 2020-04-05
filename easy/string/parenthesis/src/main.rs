// Given a string containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

// An input string is valid if:

// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
// Note that an empty string is also considered valid.

// Example 1:

// Input: "()"
// Output: true
// Example 2:

// Input: "()[]{}"
// Output: true
// Example 3:

// Input: "(]"
// Output: false
// Example 4:

// Input: "([)]"
// Output: false
// Example 5:

// Input: "{[]}"
// Output: true

fn main() {
    assert_eq!(Solution::is_valid(String::from("]")), false);
    assert_eq!(Solution::is_valid(String::from("{}")), true);
    assert_eq!(Solution::is_valid(String::from("{[]}")), true);
    assert_eq!(Solution::is_valid(String::from("")), true);
    assert_eq!(Solution::is_valid(String::from("{{{]}")), false);
}

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for ch in s.chars() {
            match ch {
                '{' | '(' | '[' => stack.push(ch),
                '}' => {
                    if stack.last() == Some(&'{') {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
                ')' => {
                    if stack.last() == Some(&'(') {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
                ']' => {
                    if stack.last() == Some(&'[') {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
                _ => return false,
            };
        }
        stack.len() == 0
    }
}
