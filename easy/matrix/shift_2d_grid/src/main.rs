// Given a 2D grid of size m x n and an integer k. You need to shift the grid k times.

// In one shift operation:

// Element at grid[i][j] moves to grid[i][j + 1].
// Element at grid[i][n - 1] moves to grid[i + 1][0].
// Element at grid[m - 1][n - 1] moves to grid[0][0].
// Return the 2D grid after applying shift operation k times.

// Example 1:

// Input: grid = [[1,2,3],[4,5,6],[7,8,9]], k = 1
// Output: [[9,1,2],[3,4,5],[6,7,8]]
// Example 2:

// Input: grid = [[3,8,1,9],[19,7,2,5],[4,6,11,10],[12,0,21,13]], k = 4
// Output: [[12,0,21,13],[3,8,1,9],[19,7,2,5],[4,6,11,10]]
// Example 3:

// Input: grid = [[1,2,3],[4,5,6],[7,8,9]], k = 9
// Output: [[1,2,3],[4,5,6],[7,8,9]]

// Constraints:

// m == grid.length
// n == grid[i].length
// 1 <= m <= 50
// 1 <= n <= 50
// -1000 <= grid[i][j] <= 1000
// 0 <= k <= 100

struct Solution;

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let m = grid.len();
        let n = grid[0].len();
        let total = m * n;
        if k == 0 || (k % total as i32 == 0 && k != 1) {
            return grid;
        }

        for i in 0..m {
            result.push(Vec::new());
            for j in 0..n {
                result[i].push(0);
            }
        }

        for i in 0..m {
            for j in 0..n {
                let new_col = (j + k as usize) % n;
                let add = (j + k as usize) / n;
                let new_row = (i + add) % m;
                result[new_row][new_col] = grid[i][j];
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 18),
        vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
    );
    assert_eq!(
        Solution::shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1),
        vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]
    )
}
