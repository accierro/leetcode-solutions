// Write a class RecentCounter to count recent requests.

// It has only one method: ping(int t), where t represents some time in milliseconds.

// Return the number of pings that have been made from 3000 milliseconds ago until now.

// Any ping with time in [t - 3000, t] will count, including the current ping.

// It is guaranteed that every call to ping uses a strictly larger value of t than before.

// Example 1:

// Input: inputs = ["RecentCounter","ping","ping","ping","ping"], inputs = [[],[1],[100],[3001],[3002]]
// Output: [null,1,2,3,3]

// Note:

// Each test case will have at most 10000 calls to ping.
// Each test case will call ping with strictly increasing values of t.
// Each call to ping will have 1 <= t <= 10^9.

use std::collections::VecDeque;

struct RecentCounter {
    queue: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        RecentCounter {
            queue: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.queue.push_front(t);
        while let Some(val) = self.queue.back() {
            if *val < t - 3000 {
                self.queue.pop_back();
            } else {
                break;
            }
        }
        self.queue.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */

fn main() {
    let mut counter = RecentCounter::new();
    counter.ping(1);
    assert_eq!(counter.ping(2), 2);
    assert_eq!(counter.ping(3), 3);
    assert_eq!(counter.ping(3002), 2);
}
