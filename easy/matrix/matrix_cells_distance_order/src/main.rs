// We are given a matrix with R rows and C columns has cells with integer coordinates (r, c), where 0 <= r < R and 0 <= c < C.

// Additionally, we are given a cell in that matrix with coordinates (r0, c0).

// Return the coordinates of all cells in the matrix, sorted by their distance from (r0, c0) from smallest distance to largest distance.  Here, the distance between two cells (r1, c1) and (r2, c2) is the Manhattan distance, |r1 - r2| + |c1 - c2|.  (You may return the answer in any order that satisfies this condition.)

// Example 1:

// Input: R = 1, C = 2, r0 = 0, c0 = 0
// Output: [[0,0],[0,1]]
// Explanation: The distances from (r0, c0) to other cells are: [0,1]
// Example 2:

// Input: R = 2, C = 2, r0 = 0, c0 = 1
// Output: [[0,1],[0,0],[1,1],[1,0]]
// Explanation: The distances from (r0, c0) to other cells are: [0,1,1,2]
// The answer [[0,1],[1,1],[0,0],[1,0]] would also be accepted as correct.
// Example 3:

// Input: R = 2, C = 3, r0 = 1, c0 = 2
// Output: [[1,2],[0,2],[1,1],[0,1],[1,0],[0,0]]
// Explanation: The distances from (r0, c0) to other cells are: [0,1,1,2,2,3]
// There are other answers that would also be accepted as correct, such as [[1,2],[1,1],[0,2],[1,0],[0,1],[0,0]].

struct Solution;

impl Solution {
    pub fn all_cells_dist_order(R: i32, C: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut result = vec![vec![r0, c0]];
        let diag = vec![r0 + c0, r0 + C - c0, c0 + R - r0, R - r0 + C - c0];
        let max_d = diag.iter().max().unwrap();

        for d in 1..*max_d + 1 {
            for x in (-d..d + 1).rev() {
                let r1 = r0 + x;
                let c1_a = c0 + d - x.abs();
                let c1_b = c0 + x.abs() - d;
                if r1 >= 0 && r1 < R {
                    if c1_a >= 0 && c1_a < C {
                        result.push(vec![r1, c1_a]);
                    }
                    if c1_b != c1_a && c1_b >= 0 && c1_b < C {
                        result.push(vec![r1, c1_b]);
                    }
                }
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::all_cells_dist_order(1, 2, 0, 0),
        vec![vec![0, 0], vec![0, 1]]
    );
    assert_eq!(
        Solution::all_cells_dist_order(2, 2, 0, 1),
        vec![vec![0, 1], vec![0, 0], vec![1, 1], vec![1, 0]]
    );
}
