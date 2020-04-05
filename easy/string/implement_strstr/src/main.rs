// Implement strStr().

// Return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.

// Example 1:

// Input: haystack = "hello", needle = "ll"
// Output: 2
// Example 2:

// Input: haystack = "aaaaa", needle = "bba"
// Output: -1
// Clarification:

// What should we return when needle is an empty string? This is a great question to ask during an interview.

// For the purpose of this problem, we will return 0 when needle is an empty string. This is consistent to C's strstr() and Java's indexOf().

struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() > haystack.len() {
            return -1;
        }
        if needle.len() == 0 {
            return 0;
        }

        for i in 0..haystack.len() - needle.len() + 1 {
            if haystack[i..i + needle.len()] == needle {
                return i as i32;
            }
        }

        -1
    }
}

fn main() {
    let str1 = String::from("hello");
    let needle = String::from("ll");

    assert_eq!(Solution::str_str(str1, needle), 2);
}
