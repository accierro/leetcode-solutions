// Given n and m which are the dimensions of a matrix initialized by zeros and given an array indices where indices[i] = [ri, ci]. For each pair of [ri, ci] you have to increment all cells in row ri and column ci by 1.

// Return the number of cells with odd values in the matrix after applying the increment to all indices.

// Example 1:

// Input: n = 2, m = 3, indices = [[0,1],[1,1]]
// Output: 6
// Explanation: Initial matrix = [[0,0,0],[0,0,0]].
// After applying first increment it becomes [[1,2,1],[0,1,0]].
// The final matrix will be [[1,3,1],[1,3,1]] which contains 6 odd numbers.
// Example 2:

// Input: n = 2, m = 2, indices = [[1,1],[0,0]]
// Output: 0
// Explanation: Final matrix = [[2,2],[2,2]]. There is no odd number in the final matrix.

// Constraints:

// 1 <= n <= 50
// 1 <= m <= 50
// 1 <= indices.length <= 100
// 0 <= indices[i][0] < n
// 0 <= indices[i][1] < m

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn odd_cells(n: i32, m: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut rows: HashMap<i32, i32> = HashMap::new();
        let mut col: HashMap<i32, i32> = HashMap::new();

        for nth in 0..n {
            rows.insert(nth, 0);
        }

        for mth in 0..m {
            col.insert(mth, 0);
        }

        for ind in indices.iter() {
            rows.entry(ind[0]).and_modify(|e| *e += 1).or_insert(1);
            col.entry(ind[1]).and_modify(|e| *e += 1).or_insert(1);
        }

        let mut result = 0;
        for nth in 0..n {
            for mth in 0..m {
                match (rows.get(&nth), col.get(&mth)) {
                    (Some(x), Some(y)) if (x + y) % 2 == 1 => result += 1,
                    (_, _) => (),
                }
            }
        }
        result
    }
}

fn main() {
    assert_eq!(Solution::odd_cells(2, 3, vec![vec![0, 1], vec![1, 1]]), 6);
    assert_eq!(Solution::odd_cells(2, 2, vec![vec![1, 1], vec![0, 0]]), 0);
}
