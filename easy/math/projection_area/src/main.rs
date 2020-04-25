// On a N * N grid, we place some 1 * 1 * 1 cubes that are axis-aligned with the x, y, and z axes.

// Each value v = grid[i][j] represents a tower of v cubes placed on top of grid cell (i, j).

// Now we view the projection of these cubes onto the xy, yz, and zx planes.

// A projection is like a shadow, that maps our 3 dimensional figure to a 2 dimensional plane.

// Here, we are viewing the "shadow" when looking at the cubes from the top, the front, and the side.

// Return the total area of all three projections.

// Example 1:

// Input: [[2]]
// Output: 5
// Example 2:

// Input: [[1,2],[3,4]]
// Output: 17
// Explanation:
// Here are the three projections ("shadows") of the shape made with each axis-aligned plane.

// Example 3:

// Input: [[1,0],[0,2]]
// Output: 8
// Example 4:

// Input: [[1,1,1],[1,0,1],[1,1,1]]
// Output: 14
// Example 5:

// Input: [[2,2,2],[2,1,2],[2,2,2]]
// Output: 21

// Note:

// 1 <= grid.length = grid[0].length <= 50
// 0 <= grid[i][j] <= 50

struct Solution;

impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        // xy - is just a length of grid sub arrays
        let mut bottom_shadow = 0;
        // xz - max of row
        let mut row_max = vec![0; n];
        let mut row_shadow = 0;
        // yz - max of column
        let mut col_max = vec![0; n];
        let mut col_shadow = 0;

        for r in 0..n {
            for c in 0..n {
                let num = grid[r][c];
                if num > 0 {
                    bottom_shadow += 1;
                    if row_max[r] < num {
                        row_max[r] = num;
                    }
                    if col_max[c] < num {
                        col_max[c] = num;
                    }
                }
            }
        }

        row_shadow = row_max.iter().sum();
        col_shadow = col_max.iter().sum();

        bottom_shadow + col_shadow + row_shadow
    }
}

fn main() {
    assert_eq!(Solution::projection_area(vec![vec![2]]), 5);
    assert_eq!(Solution::projection_area(vec![vec![1, 2], vec![3, 4]]), 17);
}
