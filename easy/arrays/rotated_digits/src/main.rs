// X is a good number if after rotating each digit individually by 180 degrees, we get a valid number that is different from X.  Each digit must be rotated - we cannot choose to leave it alone.

// A number is valid if each digit remains a digit after rotation. 0, 1, and 8 rotate to themselves; 2 and 5 rotate to each other (on this case they are rotated in a different direction, in other words 2 or 5 gets mirrored); 6 and 9 rotate to each other, and the rest of the numbers do not rotate to any other number and become invalid.

// Now given a positive number N, how many numbers X from 1 to N are good?

// Example:
// Input: 10
// Output: 4
// Explanation:
// There are four good numbers in the range [1, 10] : 2, 5, 6, 9.
// Note that 1 and 10 are not good numbers, since they remain unchanged after rotating.
// Note:

// N  will be in range [1, 10000].

struct Solution;

impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        let mut result = 0;

        for i in 0..n + 1 {
            if Self::is_valid(i) {
                result += 1;
            }
        }

        result
    }

    fn is_valid(mut n: i32) -> bool {
        let mut res = false;
        while n > 0 {
            match n % 10 {
                2 | 5 | 6 | 9 => res = true,
                3 | 4 | 7 => return false,
                _ => (),
            }
            n /= 10;
        }

        res
    }
}

fn main() {
    assert_eq!(Solution::rotated_digits(10), 4);
}
