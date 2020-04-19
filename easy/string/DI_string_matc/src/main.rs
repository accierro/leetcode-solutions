// Given a string S that only contains "I" (increase) or "D" (decrease), let N = S.length.

// Return any permutation A of [0, 1, ..., N] such that for all i = 0, ..., N-1:

// If S[i] == "I", then A[i] < A[i+1]
// If S[i] == "D", then A[i] > A[i+1]

// Example 1:

// Input: "IDID"
// Output: [0,4,1,3,2]
// Example 2:

// Input: "III"
// Output: [0,1,2,3]
// Example 3:

// Input: "DDI"
// Output: [3,2,0,1]

// Note:

// 1 <= S.length <= 10000
// S only contains characters "I" or "D".

struct Solution;

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; s.len() + 1];
        let mut left = 0;
        let mut right = s.len() as i32;
        for (i, ch) in s.chars().enumerate() {
            match ch {
                'I' => {
                    result[i] = left;
                    left += 1;
                }
                'D' => {
                    result[i] = right;
                    right -= 1;
                }
                _ => panic!("Incorrect char"),
            }
        }
        result[s.len()] = left;
        result
    }
}

fn main() {
    assert_eq!(
        Solution::di_string_match("IDID".to_string()),
        vec![0, 4, 1, 3, 2]
    );
}
