// You're given strings J representing the types of stones that are jewels, and S representing the stones you have.  Each character in S is a type of stone you have.  You want to know how many of the stones you have are also jewels.

// The letters in J are guaranteed distinct, and all characters in J and S are letters. Letters are case sensitive, so "a" is considered a different type of stone from "A".

// Example 1:

// Input: J = "aA", S = "aAAbbbb"
// Output: 3
// Example 2:

// Input: J = "z", S = "ZZ"
// Output: 0
// Note:

// S and J will consist of letters and have length at most 50.
// The characters in J are distinct.

struct Solution;

use std::collections::hash_map::Entry;
use std::collections::HashMap;

impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let mut hashtable = HashMap::new();

        for ch in j.chars() {
            hashtable.insert(ch, 0);
        }
        let mut result = 0;
        for ch in s.chars() {
            match hashtable.entry(ch) {
                Entry::Occupied(_) => result += 1,
                Entry::Vacant(_) => {}
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::num_jewels_in_stones(String::from("aA"), String::from("aAAbbbb")),
        3
    );
}
