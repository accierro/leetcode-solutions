// Given a string S, we can transform every letter individually to be lowercase or uppercase to create another string.  Return a list of all possible strings we could create.

// Examples:
// Input: S = "a1b2"
// Output: ["a1b2", "a1B2", "A1b2", "A1B2"]

// Input: S = "3z4"
// Output: ["3z4", "3Z4"]

// Input: S = "12345"
// Output: ["12345"]
// Note:

// S will be a string with length between 1 and 12.
// S will consist only of letters or digits.

struct Solution;

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut result: Vec<String> = vec![s.clone()];

        for (i, ch) in s.chars().enumerate() {
            if ch.is_ascii_alphabetic() {
                if ch.is_ascii_lowercase() {
                    let len = result.len();
                    for j in 0..len {
                        result.push(format!(
                            "{}{}{}",
                            &result[j][0..i],
                            ch.to_ascii_uppercase(),
                            &s[i + 1..]
                        ));
                    }
                } else {
                    let len = result.len();
                    for j in 0..len {
                        result.push(format!(
                            "{}{}{}",
                            &result[j][0..i],
                            ch.to_ascii_lowercase(),
                            &s[i + 1..]
                        ));
                    }
                }
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::letter_case_permutation("a1b2".to_string()),
        vec![
            "a1b2".to_string(),
            "a1B2".to_string(),
            "A1b2".to_string(),
            "A1B2".to_string()
        ]
    );
}
