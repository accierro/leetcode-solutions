struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut map: HashMap<char, i32> = HashMap::new();

        for ch in s.chars() {
            map.entry(ch).and_modify(|e| *e += 1).or_insert(1);
        }

        for ch in t.chars() {
            let res = map.entry(ch).and_modify(|e| *e -= 1).or_insert(-1);
            if *res < 0 {
                return false;
            }
        }

        for val in map.values() {
            if *val != 0 {
                return false;
            }
        }
        true
    }
}

fn main() {
    assert_eq!(
        Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
        true
    );
    assert_eq!(
        Solution::is_anagram("anagra".to_string(), "nagaram".to_string()),
        false
    );
}
