// Given a string S and a character C, return an array of integers representing the shortest distance from the character C in the string.

// Example 1:

// Input: S = "loveleetcode", C = 'e'
// Output: [3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]

// Note:

// S string length is in [1, 10000].
// C is a single character, and guaranteed to be in string S.
// All letters in S and C are lowercase.

struct Solution;

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut result = Vec::new();
        let mut indicies = Vec::new();

        for (i, ch) in s.chars().enumerate() {
            if ch == c {
                indicies.push(i);
            }
        }

        let mut closest = 0;
        for i in 0..s.len() {
            if i > indicies[closest] {
                if closest + 1 < indicies.len() {
                    if i > (indicies[closest] + indicies[closest + 1]) / 2 {
                        closest += 1;
                    }
                }
            }
            result.push((i as i32 - indicies[closest] as i32).abs());
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::shortest_to_char("loveleetcode".to_string(), 'e'),
        vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]
    );
}
