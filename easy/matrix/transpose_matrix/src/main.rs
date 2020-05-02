// Given a matrix A, return the transpose of A.

// The transpose of a matrix is the matrix flipped over it's main diagonal, switching the row and column indices of the matrix.

// Example 1:

// Input: [[1,2,3],[4,5,6],[7,8,9]]
// Output: [[1,4,7],[2,5,8],[3,6,9]]
// Example 2:

// Input: [[1,2,3],[4,5,6]]
// Output: [[1,4],[2,5],[3,6]]

// Note:

// 1 <= A.length <= 1000
// 1 <= A[0].length <= 1000

struct Solution;

impl Solution {
    pub fn transpose(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        for row in 0..a.len() {
            for col in 0..a[0].len() {
                let num = a[row][col];
                if let Some(v) = result.get_mut(col) {
                    v.push(num);
                } else {
                    result.push(vec![num]);
                }
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]),
        vec![vec![1, 4], vec![2, 5], vec![3, 6]]
    );
}
