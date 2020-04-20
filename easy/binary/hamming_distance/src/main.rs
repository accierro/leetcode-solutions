// The Hamming distance between two integers is the number of positions at which the corresponding bits are different.

// Given two integers x and y, calculate the Hamming distance.

// Note:
// 0 ≤ x, y < 231.

// Example:

// Input: x = 1, y = 4

// Output: 2

// Explanation:
// 1   (0 0 0 1)
// 4   (0 1 0 0)
//        ↑   ↑

// The above arrows point to positions where the corresponding bits are different.

struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut bx = x;
        let mut by = y;
        let mut result = 0;

        while bx != 0 || by != 0 {
            if bx & 1 != by & 1 {
                result += 1;
            }
            bx >>= 1;
            by >>= 1;
        }

        result
    }
}

// Interesting solution
// impl Solution {
//     pub fn hamming_distance(x: i32, y: i32) -> i32 {
//         return format!("{:b}", x ^ y).matches('1').count() as i32;
//     }
// }
//
// or
//
//
// impl Solution {
//     pub fn hamming_distance(x: i32, y: i32) -> i32 {
//         (x ^ y).count_ones() as i32
//     }
// }

fn main() {
    assert_eq!(Solution::hamming_distance(1, 4), 2);
}
