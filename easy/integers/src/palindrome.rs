struct Solution {
    input_out: Vec<(i32, bool)>,
}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let number_as_string = x.to_string().chars().rev().collect::<String>();
        match number_as_string.parse::<i32>() {
            Ok(val) => return x == val,
            Err(val) => return false,
        };
    }
}
