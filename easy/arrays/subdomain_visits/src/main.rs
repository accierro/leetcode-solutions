// We are given a list cpdomains of count-paired domains. We would like a list of count-paired domains, (in the same format as the input, and in any order), that explicitly counts the number of visits to each subdomain.

// Example 1:
// Input:
// ["9001 discuss.leetcode.com"]
// Output:
// ["9001 discuss.leetcode.com", "9001 leetcode.com", "9001 com"]
// Explanation:
// We only have one website domain: "discuss.leetcode.com". As discussed above, the subdomain "leetcode.com" and "com" will also be visited. So they will all be visited 9001 times.

// Example 2:
// Input:
// ["900 google.mail.com", "50 yahoo.com", "1 intel.mail.com", "5 wiki.org"]
// Output:
// ["901 mail.com","50 yahoo.com","900 google.mail.com","5 wiki.org","5 org","1 intel.mail.com","951 com"]
// Explanation:
// We will visit "google.mail.com" 900 times, "yahoo.com" 50 times, "intel.mail.com" once and "wiki.org" 5 times. For the subdomains, we will visit "mail.com" 900 + 1 = 901 times, "com" 900 + 50 + 1 = 951 times, and "org" 5 times.

// Notes:

// The length of cpdomains will not exceed 100.
// The length of each domain name will not exceed 100.
// Each address will have either 1 or 2 "." characters.
// The input count in any count-paired domain will not exceed 10000.
// The answer output can be returned in any order.

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut domains: HashMap<String, i32> = HashMap::new();
        let mut result = Vec::new();

        for s in cpdomains.iter() {
            let sub: Vec<&str> = s.split(" ").collect();
            let num = sub[0].parse::<i32>().unwrap();
            let sub_dom: Vec<&str> = sub[1].split(".").collect();
            let mut prev_str = String::new();
            for i in (0..sub_dom.len()).rev() {
                if i == sub_dom.len() - 1 {
                    prev_str = format!("{}", sub_dom[i]);
                } else {
                    prev_str = format!("{}.{}", sub_dom[i], prev_str);
                }
                domains
                    .entry(prev_str.to_string())
                    .and_modify(|e| *e += num)
                    .or_insert(num);
            }
        }

        for (key, val) in domains.iter() {
            result.push(format!("{} {}", val, key));
        }
        result
    }
}

fn main() {
    assert_eq!(
        Solution::subdomain_visits(vec![
            "900 google.mail.com".to_string(),
            "50 yahoo.com".to_string(),
            "1 intel.mail.com".to_string(),
            "5 wiki.org".to_string()
        ]),
        vec![
            "901 mail.com".to_string(),
            "50 yahoo.com".to_string(),
            "900 google.mail.com".to_string(),
            "5 wiki.org".to_string(),
            "5 org".to_string(),
            "1 intel.mail.com".to_string(),
            "951 com".to_string()
        ]
    );
}
