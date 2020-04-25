// Given two arrays arr1 and arr2, the elements of arr2 are distinct, and all elements in arr2 are also in arr1.

// Sort the elements of arr1 such that the relative ordering of items in arr1 are the same as in arr2.  Elements that don't appear in arr2 should be placed at the end of arr1 in ascending order.

// Example 1:

// Input: arr1 = [2,3,1,3,2,4,6,7,9,2,19], arr2 = [2,1,4,3,9,6]
// Output: [2,2,2,1,4,3,3,9,6,7,19]

// Constraints:

// arr1.length, arr2.length <= 1000
// 0 <= arr1[i], arr2[i] <= 1000
// Each arr2[i] is distinct.
// Each arr2[i] is in arr1.

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        let mut excessive: Vec<i32> = Vec::new();
        let mut result = Vec::new();

        for num in arr2.iter() {
            map.insert(*num, 0);
        }

        for num in arr1.iter() {
            if map.contains_key(num) {
                map.entry(*num).and_modify(|e| *e += 1);
            } else {
                excessive.push(*num);
            }
        }

        excessive.sort_unstable();

        for num in arr2.iter() {
            let mut vector = vec![*num; *map.get(num).unwrap()];
            result.append(&mut vector);
        }
        result.append(&mut excessive);
        result
    }
}

fn main() {
    assert_eq!(
        Solution::relative_sort_array(
            vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
            vec![2, 1, 4, 3, 9, 6]
        ),
        vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]
    );
}
