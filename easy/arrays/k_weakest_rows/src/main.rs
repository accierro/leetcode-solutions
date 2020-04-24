// Given a m * n matrix mat of ones (representing soldiers) and zeros (representing civilians), return the indexes of the k weakest rows in the matrix ordered from the weakest to the strongest.

// A row i is weaker than row j, if the number of soldiers in row i is less than the number of soldiers in row j, or they have the same number of soldiers but i is less than j. Soldiers are always stand in the frontier of a row, that is, always ones may appear first and then zeros.

// Example 1:

// Input: mat =
// [[1,1,0,0,0],
//  [1,1,1,1,0],
//  [1,0,0,0,0],
//  [1,1,0,0,0],
//  [1,1,1,1,1]],
// k = 3
// Output: [2,0,3]
// Explanation:
// The number of soldiers for each row is:
// row 0 -> 2
// row 1 -> 4
// row 2 -> 1
// row 3 -> 2
// row 4 -> 5
// Rows ordered from the weakest to the strongest are [2,0,3,1,4]
// Example 2:

// Input: mat =
// [[1,0,0,0],
//  [1,1,1,1],
//  [1,0,0,0],
//  [1,0,0,0]],
// k = 2
// Output: [0,2]
// Explanation:
// The number of soldiers for each row is:
// row 0 -> 1
// row 1 -> 4
// row 2 -> 1
// row 3 -> 1
// Rows ordered from the weakest to the strongest are [0,2,3,1]

// Constraints:

// m == mat.length
// n == mat[i].length
// 2 <= n, m <= 100
// 1 <= k <= m
// matrix[i][j] is either 0 or 1.

struct Solution;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut indicies: Vec<bool> = vec![true; mat.len()];

        for m in (0..mat[0].len()) {
            for n in 0..mat.len() {
                if indicies[n] && (result.len() as i32) < k {
                    match mat[n][m] {
                        0 => {
                            indicies[n] = false;
                            result.push(n as i32);
                        }
                        _ => (),
                    }
                }
            }
        }
        let mut i = 0;
        while (result.len() as i32) < k && i < indicies.len() {
            if indicies[i] {
                result.push(i as i32);
            }
            i += 1;
        }
        result
    }
}

fn main() {
    assert_eq!(
        Solution::k_weakest_rows(vec![vec![1, 0], vec![1, 0], vec![1, 0], vec![1, 1]], 4),
        vec![0, 1, 2, 3]
    );
    assert_eq!(
        Solution::k_weakest_rows(
            vec![
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 1, 1, 1]
            ],
            3
        ),
        vec![2, 0, 3]
    );
    assert_eq!(
        Solution::k_weakest_rows(
            vec![
                vec![1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1]
            ],
            1
        ),
        vec![0]
    );
}
