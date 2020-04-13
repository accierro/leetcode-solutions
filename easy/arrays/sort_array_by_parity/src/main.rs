// Given an array A of non-negative integers, return an array consisting of all the even elements of A, followed by all the odd elements of A.

// You may return any answer array that satisfies this condition.

// Example 1:

// Input: [3,1,2,4]
// Output: [2,4,3,1]
// The outputs [4,2,3,1], [2,4,1,3], and [4,2,1,3] would also be accepted.

// Note:

// 1 <= A.length <= 5000
// 0 <= A[i] <= 5000

struct Solution;

impl Solution {
    pub fn sort_array_by_parity(mut a: Vec<i32>) -> Vec<i32> {
        let mut pointer = a.len() - 1;
        let mut i = 0;
        while i < pointer {
            let left_number = a[i];
            let right_number = a[pointer];
            if left_number % 2 == 1 && right_number % 2 == 0 {
                a[i] = right_number;
                a[pointer] = left_number;
                pointer -= 1;
                i += 1;
            } else if left_number % 2 == 0 {
                i += 1;
            } else if right_number % 2 == 1 {
                pointer -= 1;
            } else {
                i += 1;
                pointer -= 1;
            }
        }
        a
    }
}

fn main() {
    assert_eq!(
        Solution::sort_array_by_parity(vec![3, 1, 2, 4]),
        [4, 2, 1, 3]
    );
    assert_eq!(Solution::sort_array_by_parity(vec![1, 3]), [1, 3]);
    assert_eq!(Solution::sort_array_by_parity(vec![0, 1, 2]), [0, 2, 1]);
}
