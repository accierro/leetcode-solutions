// Given a string text, you want to use the characters of text to form as many instances of the word "balloon" as possible.

// You can use each character in text at most once. Return the maximum number of instances that can be formed.

// Example 1:

// Input: text = "nlaebolko"
// Output: 1
// Example 2:

// Input: text = "loonbalxballpoon"
// Output: 2
// Example 3:

// Input: text = "leetcode"
// Output: 0

// Constraints:

// 1 <= text.length <= 10^4
// text consists of lower case English letters only.

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut result = std::i32::MAX;
        let mut map: HashMap<char, i32> = [('b', 0), ('a', 0), ('l', 0), ('o', 0), ('n', 0)]
            .iter()
            .cloned()
            .collect();

        for ch in text.chars() {
            if let Some(x) = map.get_mut(&ch) {
                *x += 1;
            }
        }

        for (key, val) in map.iter() {
            match key {
                'l' | 'o' => {
                    let num = val / 2;
                    if result > num {
                        result = num;
                    }
                }
                _ => {
                    if result > *val {
                        result = *val;
                    }
                }
            }
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::max_number_of_balloons("nlaebolko".to_string()), 1);
}
