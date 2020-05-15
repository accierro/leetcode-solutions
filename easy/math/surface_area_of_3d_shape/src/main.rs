// On a N * N grid, we place some 1 * 1 * 1 cubes.

// Each value v = grid[i][j] represents a tower of v cubes placed on top of grid cell (i, j).

// Return the total surface area of the resulting shapes.

// Example 1:

// Input: [[2]]
// Output: 10
// Example 2:

// Input: [[1,2],[3,4]]
// Output: 34
// Example 3:

// Input: [[1,0],[0,2]]
// Output: 16
// Example 4:

// Input: [[1,1,1],[1,0,1],[1,1,1]]
// Output: 32
// Example 5:

// Input: [[2,2,2],[2,1,2],[2,2,2]]
// Output: 46

// Note:

// 1 <= N <= 50
// 0 <= grid[i][j] <= 50

struct Solution;

impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        for m in 0..grid.len() {
            for n in 0..grid.len() {
                if grid[m][n] != 0 {
                    result += 2;
                }
                if m == 0 {
                    result += grid[m][n];
                } else {
                    result += (grid[m][n] - grid[m - 1][n]).abs()
                }
                if m == grid.len() - 1 {
                    result += grid[m][n];
                }

                if n == 0 {
                    result += grid[m][n];
                } else {
                    result += (grid[m][n] - grid[m][n - 1]).abs()
                }
                if n == grid.len() - 1 {
                    result += grid[m][n];
                }
            }
        }
        result
    }
}

fn main() {
    assert_eq!(Solution::surface_area(vec![vec![1, 2], vec![3, 4]]), 34);
    assert_eq!(Solution::surface_area(vec![vec![2]]), 10);
    assert_eq!(Solution::surface_area(vec![vec![1, 0], vec![0, 2]]), 16);
    assert_eq!(
        Solution::surface_area(vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]]),
        46
    );
}
