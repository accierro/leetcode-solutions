// A self-dividing number is a number that is divisible by every digit it contains.

// For example, 128 is a self-dividing number because 128 % 1 == 0, 128 % 2 == 0, and 128 % 8 == 0.

// Also, a self-dividing number is not allowed to contain the digit zero.

// Given a lower and upper number bound, output a list of every possible self dividing number, including the bounds if possible.

// Example 1:
// Input:
// left = 1, right = 22
// Output: [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
// Note:

// The boundaries of each input argument are 1 <= left <= right <= 10000.

struct Solution;

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut result = Vec::new();

        for i in left..right + 1 {
            let str_num = i.to_string();
            let mut count = 0;
            for ch in str_num.chars() {
                if ch != '0' && i % (ch.to_digit(10).unwrap() as i32) == 0 {
                    count += 1;
                }
            }
            if count == str_num.len() {
                result.push(i);
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::self_dividing_numbers(1, 22),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
    );
}
