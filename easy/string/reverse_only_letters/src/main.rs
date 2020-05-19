// Given a string S, return the "reversed" string where all characters that are not a letter stay in the same place, and all letters reverse their positions.

// Example 1:

// Input: "ab-cd"
// Output: "dc-ba"
// Example 2:

// Input: "a-bC-dEf-ghIj"
// Output: "j-Ih-gfE-dCba"
// Example 3:

// Input: "Test1ng-Leet=code-Q!"
// Output: "Qedo1ct-eeLg=ntse-T!"

// Note:

// S.length <= 100
// 33 <= S[i].ASCIIcode <= 122
// S doesn't contain \ or "

struct Solution;

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        if s == "" {
            return s;
        }
        let mut v: Vec<char> = s.chars().collect();

        let mut left = 0;
        let mut right = v.len() - 1;

        while left < right {
            if v[left].is_ascii_alphabetic() && v[right].is_ascii_alphabetic() {
                let temp = v[left];
                v[left] = v[right];
                v[right] = temp;
                left += 1;
                right -= 1;
            }

            if !v[left].is_ascii_alphabetic() {
                left += 1;
            }

            if !v[right].is_ascii_alphabetic() {
                right -= 1;
            }
        }

        v.into_iter().collect()
    }
}

fn main() {
    assert_eq!(
        Solution::reverse_only_letters("".to_string()),
        "".to_string()
    );
    assert_eq!(
        Solution::reverse_only_letters("ab-cd".to_string()),
        "dc-ba".to_string()
    );
}
