// You are given an array of strings words and a string chars.

// A string is good if it can be formed by characters from chars (each character can only be used once).

// Return the sum of lengths of all good strings in words.

// Example 1:

// Input: words = ["cat","bt","hat","tree"], chars = "atach"
// Output: 6
// Explanation:
// The strings that can be formed are "cat" and "hat" so the answer is 3 + 3 = 6.
// Example 2:

// Input: words = ["hello","world","leetcode"], chars = "welldonehoneyr"
// Output: 10
// Explanation:
// The strings that can be formed are "hello" and "world" so the answer is 5 + 5 = 10.

// Note:

// 1 <= words.length <= 1000
// 1 <= words[i].length, chars.length <= 100
// All strings contain lowercase English letters only.

struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut count = 0;
        let mut map = HashMap::new();

        for ch in chars.chars() {
            map.entry(ch).and_modify(|e| *e += 1).or_insert(1);
        }

        for word in words.iter() {
            let mut copy_map = map.clone();
            let valid = word.chars().all(|e| {
                if let Some(x) = copy_map.get_mut(&e) {
                    *x -= 1;
                    if *x < 0 {
                        return false;
                    }
                    return true;
                } else {
                    return false;
                }
            });
            if valid {
                count += word.len() as i32;
            }
        }

        count
    }
}

fn main() {
    assert_eq!(
        Solution::count_characters(
            vec![
                "cat".to_string(),
                "bt".to_string(),
                "hat".to_string(),
                "tree".to_string()
            ],
            "atach".to_string()
        ),
        6
    );
}
