// Given a m * n matrix of ones and zeros, return how many square submatrices have all ones.

// Example 1:

// Input: matrix =
// [
//   [0,1,1,1],
//   [1,1,1,1],
//   [0,1,1,1]
// ]
// Output: 15
// Explanation:
// There are 10 squares of side 1.
// There are 4 squares of side 2.
// There is  1 square of side 3.
// Total number of squares = 10 + 4 + 1 = 15.
// Example 2:

// Input: matrix =
// [
//   [1,0,1],
//   [1,1,0],
//   [1,1,0]
// ]
// Output: 7
// Explanation:
// There are 6 squares of side 1.
// There is 1 square of side 2.
// Total number of squares = 6 + 1 = 7.

// Constraints:

// 1 <= arr.length <= 300
// 1 <= arr[0].length <= 300
// 0 <= arr[i][j] <= 1

struct Solution;
use std::cmp;

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        let mut table = vec![vec![0; matrix[0].len()]; matrix.len()];
        for n in 0..matrix.len() {
            for m in 0..matrix[n].len() {
                if matrix[n][m] == 1 && n != 0 && m != 0 {
                    let count = cmp::min(
                        table[n][m - 1],
                        cmp::min(table[n - 1][m], table[n - 1][m - 1]),
                    );
                    table[n][m] = cmp::max(1, count + 1);
                } else {
                    table[n][m] = matrix[n][m];
                }
                result += table[n][m];
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::count_squares(vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]]),
        15
    );
}
