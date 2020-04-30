// Given words first and second, consider occurrences in some text of the form "first second third", where second comes immediately after first, and third comes immediately after second.

// For each such occurrence, add "third" to the answer, and return the answer.

// Example 1:

// Input: text = "alice is a good girl she is a good student", first = "a", second = "good"
// Output: ["girl","student"]
// Example 2:

// Input: text = "we will we will rock you", first = "we", second = "will"
// Output: ["we","rock"]

// Note:

// 1 <= text.length <= 1000
// text consists of space separated words, where each word consists of lowercase English letters.
// 1 <= first.length, second.length <= 10
// first and second consist of lowercase English letters.

struct Solution;

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let mut result = Vec::new();
        let mut splitted: Vec<&str> = text.split(" ").collect();

        for i in 0..splitted.len() - 2 {
            if splitted[i] == first.as_str() && splitted[i + 1] == second.as_str() {
                result.push(splitted[i + 2].to_string());
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::find_ocurrences(
            "alice is a good girl she is a good student".to_string(),
            "a".to_string(),
            "good".to_string()
        ),
        vec!["girl".to_string(), "student".to_string()]
    );
}
