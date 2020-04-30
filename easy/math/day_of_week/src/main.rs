// Given a date, return the corresponding day of the week for that date.

// The input is given as three integers representing the day, month and year respectively.

// Return the answer as one of the following values {"Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"}.

// Example 1:

// Input: day = 31, month = 8, year = 2019
// Output: "Saturday"
// Example 2:

// Input: day = 18, month = 7, year = 1999
// Output: "Sunday"
// Example 3:

// Input: day = 15, month = 8, year = 1993
// Output: "Sunday"

// Constraints:

// The given dates are valid dates between the years 1971 and 2100.

struct Solution;

impl Solution {
    pub fn day_of_the_week(mut day: i32, mut month: i32, mut year: i32) -> String {
        let days = vec![
            "Sunday".to_string(),
            "Monday".to_string(),
            "Tuesday".to_string(),
            "Wednesday".to_string(),
            "Thursday".to_string(),
            "Friday".to_string(),
            "Saturday".to_string(),
        ];
        if month < 3 {
            month += 12;
            year -= 1;
        }
        let w = (day + 13 * (month + 1) / 5 + year + year / 4 + 5) % 7;
        println!("{}", w);
        days[w as usize].clone()
    }
}

fn main() {
    assert_eq!(
        Solution::day_of_the_week(31, 8, 2019),
        "Saturday".to_string()
    );
    assert_eq!(
        Solution::day_of_the_week(7, 3, 2003),
        "Saturday".to_string()
    );
}
