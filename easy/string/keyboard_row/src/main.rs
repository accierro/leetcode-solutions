// Given a List of words, return the words that can be typed using letters of alphabet on only one row's of American keyboard like the image below.
// Example:

// Input: ["Hello", "Alaska", "Dad", "Peace"]
// Output: ["Alaska", "Dad"]

// Note:

// You may use one character in the keyboard more than once.
// You may assume the input string will only contain letters of alphabet.

struct Solution;

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let aplhabet = [
            2, 1, 1, 2, 3, 2, 2, 2, 3, 2, 2, 2, 1, 1, 3, 3, 3, 3, 2, 3, 3, 1, 3, 1, 3, 1,
        ];
        let mut result = Vec::new();

        for word in words.iter() {
            let mut num: i32 = 0;
            let mut valid = true;
            for ch in word.chars() {
                let code = ch.to_ascii_lowercase() as u8 - 97;
                if num == 0 {
                    num = aplhabet[code as usize];
                }
                if num != aplhabet[code as usize] {
                    valid = false;
                    break;
                }
            }
            if valid {
                result.push(word.to_string());
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::find_words(vec![
            "Hello".to_string(),
            "Alaska".to_string(),
            "Dad".to_string(),
            "Peace".to_string()
        ]),
        vec!["Alaska".to_string(), "Dad".to_string()]
    );
}
