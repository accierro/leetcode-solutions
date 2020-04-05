// Write a function to find the longest common prefix string amongst an array of strings.

// If there is no common prefix, return an empty string "".

// Example 1:

// Input: ["flower","flow","flight"]
// Output: "fl"
// Example 2:

// Input: ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.
// Note:

// All given inputs are in lowercase letters a-z.

fn main() {
    let mut strs = vec!["flower", "flow", "flight"];
    let mut result = String::from("");
    if (strs.len() == 0) {
        return result;
    }
    strs.sort();

    let (index, mut second_str_iter) = if strs[0].len() > strs[strs.len() - 1].len() {
        (0, strs[strs.len() - 1].chars())
    } else {
        (strs.len() - 1, strs[0].chars())
    };

    for ch in strs[index].chars() {
        if Some(ch) == second_str_iter.next() {
            result.push(ch);
        } else {
            break;
        }
    }

    result
}
