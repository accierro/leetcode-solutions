// Given a string s formed by digits ('0' - '9') and '#' . We want to map s to English lowercase characters as follows:

// Characters ('a' to 'i') are represented by ('1' to '9') respectively.
// Characters ('j' to 'z') are represented by ('10#' to '26#') respectively.
// Return the string formed after mapping.

// It's guaranteed that a unique mapping will always exist.

// Example 1:

// Input: s = "10#11#12"
// Output: "jkab"
// Explanation: "j" -> "10#" , "k" -> "11#" , "a" -> "1" , "b" -> "2".
// Example 2:

// Input: s = "1326#"
// Output: "acz"
// Example 3:

// Input: s = "25#"
// Output: "y"
// Example 4:

// Input: s = "12345678910#11#12#13#14#15#16#17#18#19#20#21#22#23#24#25#26#"
// Output: "abcdefghijklmnopqrstuvwxyz"

// Constraints:

// 1 <= s.length <= 1000
// s[i] only contains digits letters ('0'-'9') and '#' letter.
// s will be valid string such that mapping is always possible.

struct Solution;

impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let mut result = String::new();
        let mut cursor = 0;
        let mut i: usize = 0;
        while i < s.len() {
            if i + 2 < s.len() && s.get(i + 2..i + 3).unwrap() == "#" {
                result.push((s.get(i..i + 2).unwrap().parse::<u8>().unwrap() + 96) as char);
                i += 3;
            } else {
                result.push((s.get(i..i + 1).unwrap().parse::<u8>().unwrap() + 96) as char);
                i += 1;
            }
        }
        result
    }
}

fn main() {
    assert_eq!(
        Solution::freq_alphabets("126#".to_string()),
        "az".to_string()
    );
    assert_eq!(Solution::freq_alphabets("25#".to_string()), "y".to_string());
    assert_eq!(
        Solution::freq_alphabets("10#11#12".to_string()),
        "jkab".to_string()
    );
    assert_eq!(
        Solution::freq_alphabets("1326#".to_string()),
        "acz".to_string()
    );
    assert_eq!(
        Solution::freq_alphabets("1326#1".to_string()),
        "acza".to_string()
    );
}
