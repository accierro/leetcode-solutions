// Given two binary strings, return their sum (also a binary string).

// The input strings are both non-empty and contains only characters 1 or 0.

// Example 1:

// Input: a = "11", b = "1"
// Output: "100"
// Example 2:

// Input: a = "1010", b = "1011"
// Output: "10101"

struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (largest, mut smallest) = if a.len() > b.len() {
            (a.chars().rev(), b.chars().rev())
        } else {
            (b.chars().rev(), a.chars().rev())
        };
        let mut result = String::from("");

        let mut carrier = false;
        for ch in largest {
            match smallest.next() {
                Some(val) => {
                    if ch == '1' && val == '1' {
                        if carrier {
                            result.insert(0, '1');
                        } else {
                            result.insert(0, '0');
                        }
                        carrier = true;
                    } else if ch == '0' && val == '0' {
                        if carrier {
                            result.insert(0, '1');
                        } else {
                            result.insert(0, '0');
                        }
                        carrier = false;
                    } else {
                        if carrier {
                            result.insert(0, '0');
                            carrier = true;
                        } else {
                            result.insert(0, '1');
                            carrier = false;
                        }
                    }
                }
                None => {
                    if carrier {
                        if ch == '1' {
                            result.insert(0, '0');
                            carrier = true;
                        } else {
                            result.insert(0, '1');
                            carrier = false;
                        }
                    } else {
                        result.insert(0, ch);
                    }
                }
            };
        }
        if carrier {
            result.insert(0, '1');
        }
        result
    }
}

fn main() {
    assert_eq!(
        Solution::add_binary(String::from("10101"), String::from("1010")),
        "11111"
    );
    assert_eq!(
        Solution::add_binary(String::from("11"), String::from("1")),
        "100"
    );
    assert_eq!(
        Solution::add_binary(String::from("1010"), String::from("1011")),
        "10101"
    );
}
