// Given an array of string words. Return all strings in words which is substring of another word in any order.

// String words[i] is substring of words[j], if can be obtained removing some characters to left and/or right side of words[j].

// Example 1:

// Input: words = ["mass","as","hero","superhero"]
// Output: ["as","hero"]
// Explanation: "as" is substring of "mass" and "hero" is substring of "superhero".
// ["hero","as"] is also a valid answer.
// Example 2:

// Input: words = ["leetcode","et","code"]
// Output: ["et","code"]
// Explanation: "et", "code" are substring of "leetcode".
// Example 3:

// Input: words = ["blue","green","bu"]
// Output: []

// Constraints:

// 1 <= words.length <= 100
// 1 <= words[i].length <= 30
// words[i] contains only lowercase English letters.
// It's guaranteed that words[i] will be unique.

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn string_matching(mut words: Vec<String>) -> Vec<String> {
        let mut result = HashSet::new();

        words.sort_by(|a, b| a.len().cmp(&b.len()));

        for i in 1..words.len() {
            for j in 0..i {
                if result.contains(&words[j]) {
                    continue;
                }
                if words[i].contains(&words[j]) {
                    result.insert(words[j].clone());
                }
            }
        }

        result.into_iter().collect()
    }
}

fn main() {
    assert_eq!(
        Solution::string_matching(vec![
            "leetcode".to_string(),
            "et".to_string(),
            "code".to_string(),
            "leetcoder".to_string()
        ]),
        vec!["et".to_string(), "code".to_string()]
    );
}
