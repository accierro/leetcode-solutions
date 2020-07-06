// Given a rows * columns matrix mat of ones and zeros, return how many submatrices have all ones.

// Example 1:

// Input: mat = [[1,0,1],
//               [1,1,0],
//               [1,1,0]]
// Output: 13
// Explanation:
// There are 6 rectangles of side 1x1.
// There are 2 rectangles of side 1x2.
// There are 3 rectangles of side 2x1.
// There is 1 rectangle of side 2x2.
// There is 1 rectangle of side 3x1.
// Total number of rectangles = 6 + 2 + 3 + 1 + 1 = 13.
// Example 2:

// Input: mat = [[0,1,1,0],
//               [0,1,1,1],
//               [1,1,1,0]]
// Output: 24
// Explanation:
// There are 8 rectangles of side 1x1.
// There are 5 rectangles of side 1x2.
// There are 2 rectangles of side 1x3.
// There are 4 rectangles of side 2x1.
// There are 2 rectangles of side 2x2.
// There are 2 rectangles of side 3x1.
// There is 1 rectangle of side 3x2.
// Total number of rectangles = 8 + 5 + 2 + 4 + 2 + 2 + 1 = 24.
// Example 3:

// Input: mat = [[1,1,1,1,1,1]]
// Output: 21
// Example 4:

// Input: mat = [[1,0,1],[0,1,0],[1,0,1]]
// Output: 5

// Constraints:

// 1 <= rows <= 150
// 1 <= columns <= 150
// 0 <= mat[i][j] <= 1

struct Solution;

use std::cmp;
impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        let mut nums: Vec<Vec<i32>> = vec![vec![0; mat[0].len()]; mat.len()];

        for n in 0..mat.len() {
            let mut h = vec![1; mat[n].len()];
            for m in n..mat.len() {
                for k in 0..mat[n].len() {
                    h[k] &= mat[m][k];
                }
                count += Solution::countOneRow(&h);
            }
        }

        count
    }

    pub fn countOneRow(mut arr: &Vec<i32>) -> i32 {
        let mut res = 0;
        let mut length = 0;

        for n in 0..arr.len() {
            length = if arr[n] == 1 { length + 1 } else { 0 };
            res += length;
        }

        res
    }
}

fn main() {
    assert_eq!(
        Solution::num_submat(vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]]),
        13
    );
}
