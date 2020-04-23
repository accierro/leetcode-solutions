// Given a string, you need to reverse the order of characters in each word within a sentence while still preserving whitespace and initial word order.

// Example 1:
// Input: "Let's take LeetCode contest"
// Output: "s'teL ekat edoCteeL tsetnoc"
// Note: In the string, each word is separated by single space and there will not be any extra space in the string.

struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut result = String::new();
        let mut stack = Vec::new();

        for (i, ch) in s.chars().enumerate() {
            if ch == ' ' || i == s.len() - 1 {
                if i == s.len() - 1 {
                    stack.push(ch);
                }
                while stack.len() > 0 {
                    result.push(stack.pop().unwrap());
                }
                if ch == ' ' {
                    result.push(' ');
                }
            } else {
                stack.push(ch);
            }
        }
        result
    }
}

fn main() {
    assert_eq!(
        Solution::reverse_words("Let's take LeetCode contest".to_string()),
        "s'teL ekat edoCteeL tsetnoc".to_string()
    );
}
