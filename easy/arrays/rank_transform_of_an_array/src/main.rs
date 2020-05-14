// Given an array of integers arr, replace each element with its rank.

// The rank represents how large the element is. The rank has the following rules:

// Rank is an integer starting from 1.
// The larger the element, the larger the rank. If two elements are equal, their rank must be the same.
// Rank should be as small as possible.

// Example 1:

// Input: arr = [40,10,20,30]
// Output: [4,1,2,3]
// Explanation: 40 is the largest element. 10 is the smallest. 20 is the second smallest. 30 is the third smallest.
// Example 2:

// Input: arr = [100,100,100]
// Output: [1,1,1]
// Explanation: Same elements share the same rank.
// Example 3:

// Input: arr = [37,12,28,9,100,56,80,5,12]
// Output: [5,3,4,2,8,6,7,1,3]

// Constraints:

// 0 <= arr.length <= 105
// -109 <= arr[i] <= 109

struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; arr.len()];
        let mut tree: BTreeMap<i32, Vec<usize>> = BTreeMap::new();

        for (i, n) in arr.iter().enumerate() {
            tree.entry(*n).and_modify(|e| e.push(i)).or_insert(vec![i]);
        }

        let mut rank = 1;
        for (key, val) in tree.iter() {
            for n in val.iter() {
                result[*n] = rank;
            }
            rank += 1;
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::array_rank_transform(vec![40, 10, 20, 30]),
        vec![4, 1, 2, 3]
    );
}
