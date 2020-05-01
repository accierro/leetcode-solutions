// A matrix is Toeplitz if every diagonal from top-left to bottom-right has the same element.

// Now given an M x N matrix, return True if and only if the matrix is Toeplitz.

// Example 1:

// Input:
// matrix = [
//   [1,2,3,4],
//   [5,1,2,3],
//   [9,5,1,2]
// ]
// Output: True
// Explanation:
// In the above grid, the diagonals are:
// "[9]", "[5, 5]", "[1, 1, 1]", "[2, 2, 2]", "[3, 3]", "[4]".
// In each diagonal all elements are the same, so the answer is True.
// Example 2:

// Input:
// matrix = [
//   [1,2],
//   [2,2]
// ]
// Output: False
// Explanation:
// The diagonal "[1, 2]" has different elements.

// Note:

// matrix will be a 2D array of integers.
// matrix will have a number of rows and columns in range [1, 20].
// matrix[i][j] will be integers in range [0, 99].

struct Solution;

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        // 5 4 3 2 1 0
        // 6
        // 7
        let rows = matrix.len() as usize;
        let col = matrix[0].len() as usize;
        let d = rows + col - 1;
        for i in 0..d {
            let mut m: usize = if i < col { 0 } else { i - col };
            let mut n: usize = if i < col { i } else { 0 };
            // println!("{} {} {}", i, m, n);
            let num = matrix[m][n];
            while m < rows as usize && n < col as usize {
                if matrix[m][n] != num {
                    return false;
                }
                m += 1;
                n += 1;
            }
        }

        true
    }
}

fn main() {
    // assert_eq!(
    //     Solution::is_toeplitz_matrix(vec![vec![1, 2, 3, 4], vec![5, 1, 2, 3], vec![9, 5, 1, 2]]),
    //     true
    // );
    // assert_eq!(
    //     Solution::is_toeplitz_matrix(vec![vec![1, 2], vec![2, 2]]),
    //     false
    // );
    // assert_eq!(Solution::is_toeplitz_matrix(vec![vec![65, 98, 57]]), true);
    assert_eq!(
        Solution::is_toeplitz_matrix(vec![vec![33, 72, 44, 89]]),
        true
    );
}
