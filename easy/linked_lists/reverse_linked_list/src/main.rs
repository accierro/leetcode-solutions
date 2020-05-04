// Reverse a singly linked list.

// Example:

// Input: 1->2->3->4->5->NULL
// Output: 5->4->3->2->1->NULL
// Follow up:

// A linked list can be reversed either iteratively or recursively. Could you implement both?

struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut current = &head;

        while let Some(node) = current {
            result = Some(Box::new(ListNode {
                val: node.val,
                next: result,
            }));
            current = &node.next;
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::reverse_list(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None }))
            }))
        }))),
        Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 1, next: None }))
            }))
        }))
    );
}
