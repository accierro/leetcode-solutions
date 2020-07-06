struct Solution;

impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut result = vec![0];

        for i in 1..num + 1 {
            result.push(result[i as usize >> 1] + (i & 1));
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::count_bits(0), vec![0]);
    assert_eq!(Solution::count_bits(4), vec![0, 1, 1, 2, 1]);
    assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
}
