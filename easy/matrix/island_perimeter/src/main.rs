// You are given a map in form of a two-dimensional integer grid where 1 represents land and 0 represents water.

// Grid cells are connected horizontally/vertically (not diagonally). The grid is completely surrounded by water, and there is exactly one island (i.e., one or more connected land cells).

// The island doesn't have "lakes" (water inside that isn't connected to the water around the island). One cell is a square with side length 1. The grid is rectangular, width and height don't exceed 100. Determine the perimeter of the island.

// Example:

// Input:
// [[0,1,0,0],
//  [1,1,1,0],
//  [0,1,0,0],
//  [1,1,0,0]]

// Output: 16

struct Solution;

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] == 1 {
                    if row as i32 - 1 < 0 || grid[row - 1][col] == 0 {
                        result += 1;
                    }
                    if row + 1 >= grid.len() || grid[row + 1][col] == 0 {
                        result += 1;
                    }
                    if col as i32 - 1 < 0 || grid[row][col - 1] == 0 {
                        result += 1;
                    }
                    if col + 1 >= grid[row].len() || grid[row][col + 1] == 0 {
                        result += 1;
                    }
                }
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::island_perimeter(vec![
            vec![0, 1, 0, 0],
            vec![1, 1, 1, 0],
            vec![0, 1, 0, 0],
            vec![1, 1, 0, 0]
        ]),
        16
    );
}
