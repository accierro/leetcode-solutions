struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let mut result = 0;
        let char_vec: Vec<char> = s.chars().collect();

        for (i, ch) in char_vec.iter().enumerate() {
            result += Solution::check_polindromes(&char_vec, i as i32, i as i32);
            result += Solution::check_polindromes(&char_vec, i as i32, i as i32 + 1);
        }

        result
    }

    pub fn check_polindromes(arr: &Vec<char>, mut start: i32, mut end: i32) -> i32 {
        let mut count = 0;
        while start >= 0 && (end as usize) < arr.len() && arr[start as usize] == arr[end as usize] {
            count += 1;
            start -= 1;
            end += 1;
        }
        count
    }
}

fn main() {
    assert_eq!(Solution::count_substrings("aaa".to_string()), 6);
}
