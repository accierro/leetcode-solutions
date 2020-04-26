// Given a binary tree, each node has value 0 or 1.  Each root-to-leaf path represents a binary number starting with the most significant bit.  For example, if the path is 0 -> 1 -> 1 -> 0 -> 1, then this could represent 01101 in binary, which is 13.

// For all leaves in the tree, consider the numbers represented by the path from the root to that leaf.

// Return the sum of these numbers.

// Example 1:

// Input: [1,0,1,0,1,0,1]
// Output: 22
// Explanation: (100) + (101) + (110) + (111) = 4 + 5 + 6 + 7 = 22

// Note:

// The number of nodes in the tree is between 1 and 1000.
// node.val is 0 or 1.
// The answer will not exceed 2^31 - 1.

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let _root = root.unwrap();
        if _root.borrow().left.is_none() && _root.borrow().right.is_none() {
            return _root.borrow().val;
        }
        let mut sum = 0;
        let val = _root.borrow().val;

        if _root.borrow().left.is_some() {
            sum += Self::go_next(_root.borrow().left.clone(), val);
        }

        if _root.borrow().right.is_some() {
            sum += Self::go_next(_root.borrow().right.clone(), val);
        }

        sum
    }

    pub fn go_next(root: Option<Rc<RefCell<TreeNode>>>, previous: i32) -> i32 {
        let _root = root.unwrap();
        let val = _root.borrow().val;

        if _root.borrow().left.is_none() && _root.borrow().right.is_none() {
            return (previous << 1) + val;
        }

        let mut sum = 0;
        if _root.borrow().left.is_some() {
            sum += Self::go_next(_root.borrow().left.clone(), (previous << 1) + val);
        }

        if _root.borrow().right.is_some() {
            sum += Self::go_next(_root.borrow().right.clone(), (previous << 1) + val);
        }

        sum
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            }))),
        }))),
        right: None,
    })));
    let root2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }))),
    })));
    assert_eq!(
        Solution::sum_root_to_leaf(Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: None,
        })))),
        0
    );
    assert_eq!(Solution::sum_root_to_leaf(root), 13);
    assert_eq!(Solution::sum_root_to_leaf(root2), 16);
}
