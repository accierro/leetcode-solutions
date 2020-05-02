// We are given two sentences A and B.  (A sentence is a string of space separated words.  Each word consists only of lowercase letters.)

// A word is uncommon if it appears exactly once in one of the sentences, and does not appear in the other sentence.

// Return a list of all uncommon words.

// You may return the list in any order.

// Example 1:

// Input: A = "this apple is sweet", B = "this apple is sour"
// Output: ["sweet","sour"]
// Example 2:

// Input: A = "apple apple", B = "banana"
// Output: ["banana"]

// Note:

// 0 <= A.length <= 200
// 0 <= B.length <= 200
// A and B both contain only spaces and lowercase letters

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn uncommon_from_sentences(a: String, b: String) -> Vec<String> {
        let mut result = Vec::new();
        let mut exclusion = HashSet::new();
        let mut set1 = HashSet::new();
        let mut set2 = HashSet::new();

        for word in a.split_whitespace() {
            if set1.contains(word) {
                exclusion.insert(word);
            } else {
                set1.insert(word.to_string());
            }
        }

        for word in b.split_whitespace() {
            if set2.contains(word) {
                exclusion.insert(word);
            } else {
                set2.insert(word.to_string());
            }
        }

        let diff: HashSet<&str> = set1
            .symmetric_difference(&set2)
            .map(String::as_ref)
            .collect();
        for i in diff.difference(&exclusion) {
            result.push(i.to_string());
        }
        result
    }
}

fn main() {
    assert_eq!(
        Solution::uncommon_from_sentences(
            "this apple is sweet".to_string(),
            "this apple is sour".to_string()
        ),
        vec!["sweet", "sour"]
    );
    assert_eq!(
        Solution::uncommon_from_sentences("apple apple".to_string(), "banana".to_string()),
        vec!["banana"]
    );
}
