struct Solution {
    input_out: Vec<(i32, i32)>,
}

impl Solution {
    pub fn mine_reverse(x: i32) -> i32 {
        let s: String = x.to_string();
        if x < 0 {
            let reversed: String = s[1..].chars().rev().collect();
            let reversed: String = format!("-{}", reversed);
            match reversed.parse::<i32>() {
                Ok(val) => return val,
                Err(val) => return 0,
            }
        }
        let reversed: String = s.chars().rev().collect();
        match reversed.parse::<i32>() {
            Ok(val) => return val,
            Err(val) => return 0,
        }
    }

    pub fn reverse(mut x: i32) -> i32 {
        let mut rev: i32 = 0;
        while x != 0 {
            let pop: i32 = x % 10;
            x = x / 10;
            if rev > std::i32::MAX / 10 || (rev == std::i32::MAX && pop > 7) {
                return 0;
            }
            if rev < std::i32::MIN / 10 || (rev == std::i32::MIN && pop < -8) {
                return 0;
            }
            rev = rev * 10 + pop;
        }
        return rev;
    }
}

fn main() {
    let sol = Solution {
        input_out: vec![(132, 231)],
    };
    assert_eq!(
        Solution::mine_reverse(sol.input_out[0].0),
        sol.input_out[0].1
    );
    assert_eq!(Solution::reverse(sol.input_out[0].0), sol.input_out[0].1);
    println!("{} - passed", sol.input_out[0].0);
}
