struct Solution;

impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut result = vec![0];

        let mut current_max = 0;
        for i in 1..num + 1 {
            if result[(i as usize) - 1] > current_max {
                current_max += 1;
                result.push(1);
            } else {
                result.push(result[(i as usize) - 1] + 1);
            }
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::count_bits(0), vec![0]);
    assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
}
