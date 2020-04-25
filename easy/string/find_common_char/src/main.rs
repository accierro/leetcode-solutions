// Given an array A of strings made only from lowercase letters, return a list of all characters that show up in all strings within the list (including duplicates).  For example, if a character occurs 3 times in all strings but not 4 times, you need to include that character three times in the final answer.

// You may return the answer in any order.

// Example 1:

// Input: ["bella","label","roller"]
// Output: ["e","l","l"]
// Example 2:

// Input: ["cool","lock","cook"]
// Output: ["c","o"]

// Note:

// 1 <= A.length <= 100
// 1 <= A[i].length <= 100
// A[i][j] is a lowercase letter

struct Solution;

impl Solution {
    pub fn common_chars(a: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();
        let mut count = [std::u8::MAX; 26];
        for word in a.iter() {
            let mut local = [0; 26];
            for ch in word.chars() {
                let num = (ch as u8) - 97;
                local[num as usize] += 1;
            }
            for i in 0..26 {
                count[i] = std::cmp::min(local[i], count[i]);
            }
        }

        for i in 0..26 {
            if count[i] != 0 {
                for j in 0..count[i] {
                    let mut string = String::new();
                    string.push((i as u8 + 97) as char);
                    result.push(string);
                }
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::common_chars(vec![
            "bellaz".to_string(),
            "label".to_string(),
            "roller".to_string()
        ]),
        ["e".to_string(), "l".to_string(), "l".to_string()]
    );
}
