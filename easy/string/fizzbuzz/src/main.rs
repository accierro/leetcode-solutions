// Write a program that outputs the string representation of numbers from 1 to n.

// But for multiples of three it should output “Fizz” instead of the number and for the multiples of five output “Buzz”. For numbers which are multiples of both three and five output “FizzBuzz”.

// Example:

// n = 15,

// Return:
// [
//     "1",
//     "2",
//     "Fizz",
//     "4",
//     "Buzz",
//     "Fizz",
//     "7",
//     "8",
//     "Fizz",
//     "Buzz",
//     "11",
//     "Fizz",
//     "13",
//     "14",
//     "FizzBuzz"
// ]

struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result = Vec::new();

        for i in 1..n + 1 {
            if i % 3 == 0 && i % 5 == 0 {
                result.push("FizzBuzz".to_string());
            } else if i % 3 == 0 {
                result.push("Fizz".to_string());
            } else if i % 5 == 0 {
                result.push("Buzz".to_string());
            } else {
                result.push(i.to_string());
            }
        }

        result
    }
}

fn main() {
    println!("{:?}", Solution::fizz_buzz(15));
}
