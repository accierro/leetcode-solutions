// Write a function that reverses a string. The input string is given as an array of characters char[].

// Do not allocate extra space for another array, you must do this by modifying the input array in-place with O(1) extra memory.

// You may assume all the characters consist of printable ascii characters.

// Example 1:

// Input: ["h","e","l","l","o"]
// Output: ["o","l","l","e","h"]
// Example 2:

// Input: ["H","a","n","n","a","h"]
// Output: ["h","a","n","n","a","H"]

struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let n = s.len();
        let mid = n / 2;

        for i in 0..mid {
            let temp = s[i];
            s[i] = s[n - i - 1];
            s[n - i - 1] = temp;
        }
    }
}

fn main() {
    let mut input = vec!['h', 'e', 'l', 'l', 'o'];
    Solution::reverse_string(&mut input);
    assert_eq!(input, vec!['o', 'l', 'l', 'e', 'h']);
}
