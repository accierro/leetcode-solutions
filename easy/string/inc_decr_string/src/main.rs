// Given a string s. You should re-order the string using the following algorithm:

// Pick the smallest character from s and append it to the result.
// Pick the smallest character from s which is greater than the last appended character to the result and append it.
// Repeat step 2 until you cannot pick more characters.
// Pick the largest character from s and append it to the result.
// Pick the largest character from s which is smaller than the last appended character to the result and append it.
// Repeat step 5 until you cannot pick more characters.
// Repeat the steps from 1 to 6 until you pick all characters from s.
// In each step, If the smallest or the largest character appears more than once you can choose any occurrence and append it to the result.

// Return the result string after sorting s with this algorithm.

// Example 1:

// Input: s = "aaaabbbbcccc"
// Output: "abccbaabccba"
// Explanation: After steps 1, 2 and 3 of the first iteration, result = "abc"
// After steps 4, 5 and 6 of the first iteration, result = "abccba"
// First iteration is done. Now s = "aabbcc" and we go back to step 1
// After steps 1, 2 and 3 of the second iteration, result = "abccbaabc"
// After steps 4, 5 and 6 of the second iteration, result = "abccbaabccba"
// Example 2:

// Input: s = "rat"
// Output: "art"
// Explanation: The word "rat" becomes "art" after re-ordering it with the mentioned algorithm.
// Example 3:

// Input: s = "leetcode"
// Output: "cdelotee"
// Example 4:

// Input: s = "ggggggg"
// Output: "ggggggg"
// Example 5:

// Input: s = "spo"
// Output: "ops"

// Constraints:

// 1 <= s.length <= 500
// s contains only lower-case English letters.

struct Solution;

impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut letter_arr: [i32; 28] = [0; 28];
        let mut result = String::new();
        for ch in s.chars() {
            letter_arr[(ch as usize) - 97] += 1;
        }

        let mut count = 0;
        while count < s.len() {
            for (index, mut num) in letter_arr.iter_mut().enumerate() {
                if *num != 0 {
                    result.push((index as u8 + 97) as char);
                    *num -= 1;
                    count += 1;
                }
            }
            for (index, mut num) in letter_arr.iter_mut().rev().enumerate() {
                if *num != 0 {
                    result.push(((27 - index as u8) + 97) as char);
                    *num -= 1;
                    count += 1;
                }
            }
        }
        result
    }
}

fn main() {
    assert_eq!(
        Solution::sort_string("aaaabbbbcccc".to_string()),
        "abccbaabccba".to_string()
    );
    assert_eq!(Solution::sort_string("rat".to_string()), "art".to_string());
    assert_eq!(Solution::sort_string("t".to_string()), "t".to_string());
}
