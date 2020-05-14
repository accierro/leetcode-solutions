// Let's define a function f(s) over a non-empty string s, which calculates the frequency of the smallest character in s. For example, if s = "dcce" then f(s) = 2 because the smallest character is "c" and its frequency is 2.

// Now, given string arrays queries and words, return an integer array answer, where each answer[i] is the number of words such that f(queries[i]) < f(W), where W is a word in words.

// Example 1:

// Input: queries = ["cbd"], words = ["zaaaz"]
// Output: [1]
// Explanation: On the first query we have f("cbd") = 1, f("zaaaz") = 3 so f("cbd") < f("zaaaz").
// Example 2:

// Input: queries = ["bbb","cc"], words = ["a","aa","aaa","aaaa"]
// Output: [1,2]
// Explanation: On the first query only f("bbb") < f("aaaa"). On the second query both f("aaa") and f("aaaa") are both > f("cc").

// Constraints:

// 1 <= queries.length <= 2000
// 1 <= words.length <= 2000
// 1 <= queries[i].length, words[i].length <= 10
// queries[i][j], words[i][j] are English lowercase letters.

struct Solution;

impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let mut frequency: Vec<i32> = words.iter().map(|e| Self::smallest_frequency(e)).collect();
        frequency.sort();
        let queries_frequency: Vec<i32> = queries
            .iter()
            .map(|e| Self::smallest_frequency(e))
            .collect();
        println!("{:?}", queries_frequency);
        println!("{:?}", frequency);
        queries_frequency
            .iter()
            .map(|e| match frequency.binary_search(e) {
                Ok(val) => return (frequency.len() - 1 - val) as i32,
                Err(val) => {
                    return (frequency.len() - val) as i32;
                }
            })
            .collect()
    }

    fn smallest_frequency(word: &str) -> i32 {
        let mut arr = [0; 26];

        for ch in word.chars() {
            arr[(ch as u8 - 97) as usize] += 1;
        }

        for n in arr.iter() {
            if *n != 0 {
                return *n;
            }
        }

        0
    }
}

fn main() {
    assert_eq!(
        Solution::num_smaller_by_frequency(vec!["cbd".to_string()], vec!["zaaaz".to_string()]),
        [1]
    );
    assert_eq!(
        Solution::num_smaller_by_frequency(
            vec!["bbb".to_string(), "cc".to_string()],
            vec![
                "a".to_string(),
                "aa".to_string(),
                "aaa".to_string(),
                "aaaa".to_string()
            ]
        ),
        [1, 2]
    );
}
