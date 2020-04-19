// Given a m * n matrix of distinct numbers, return all lucky numbers in the matrix in any order.

// A lucky number is an element of the matrix such that it is the minimum element in its row and maximum in its column.

// Example 1:

// Input: matrix = [[3,7,8],[9,11,13],[15,16,17]]
// Output: [15]
// Explanation: 15 is the only lucky number since it is the minimum in its row and the maximum in its column
// Example 2:

// Input: matrix = [[1,10,4,2],[9,3,8,7],[15,16,17,12]]
// Output: [12]
// Explanation: 12 is the only lucky number since it is the minimum in its row and the maximum in its column.
// Example 3:

// Input: matrix = [[7,8],[1,2]]
// Output: [7]

// Constraints:

// m == mat.length
// n == mat[i].length
// 1 <= n, m <= 50
// 1 <= matrix[i][j] <= 10^5.
// All elements in the matrix are distinct.

use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut min = vec![std::i32::MAX; matrix.len()];
        let mut max = vec![std::i32::MIN; matrix[0].len()];
        let mut set: HashSet<i32> = HashSet::new();

        for m in 0..matrix.len() {
            for n in 0..matrix[m].len() {
                let num = matrix[m][n];
                if num < min[m] {
                    min[m] = num;
                }
                if num > max[n] {
                    max[n] = num;
                }
            }
        }

        for num in min.iter() {
            if !set.insert(*num) {
                result.push(*num);
            }
        }

        for num in max.iter() {
            if !set.insert(*num) {
                result.push(*num)
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]],),
        vec![15]
    );
    assert_eq!(
        Solution::lucky_numbers(vec![vec![76618, 42558, 65788, 20503, 29400, 54116]]),
        vec![20503]
    );
    assert_eq!(
        Solution::lucky_numbers(vec![
            vec![1, 10, 4, 2],
            vec![9, 3, 8, 7],
            vec![15, 16, 17, 12]
        ],),
        vec![12]
    );
}
