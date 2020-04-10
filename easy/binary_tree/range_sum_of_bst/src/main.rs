// Given the root node of a binary search tree, return the sum of values of all nodes with value between L and R (inclusive).

// The binary search tree is guaranteed to have unique values.

// Example 1:

// Input: root = [10,5,15,3,7,null,18], L = 7, R = 15
// Output: 32
// Example 2:

// Input: root = [10,5,15,3,7,13,18,1,null,6], L = 6, R = 10
// Output: 23

// Note:

// The number of nodes in the tree is at most 10000.
// The final answer is guaranteed to be less than 2^31.

// Definition for a binary tree node.
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
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
        let mut sum = 0;
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        match &root {
            Some(x) => stack.push(Rc::clone(&x)),
            None => return 0,
        };

        while stack.len() > 0 {
            let current = stack.pop();
            if current.is_some() {
                let x = current.unwrap();
                let tree_node = x.borrow();
                if tree_node.val >= l && tree_node.val <= r {
                    sum += tree_node.val;
                }
                if tree_node.left.is_some() {
                    let y = tree_node.left.as_ref().unwrap();
                    if l <= tree_node.val {
                        stack.push(Rc::clone(&y));
                    }
                }
                if tree_node.right.is_some() {
                    let y = tree_node.right.as_ref().unwrap();
                    if tree_node.val <= r {
                        stack.push(Rc::clone(&y));
                    }
                }
            }
        }
        sum
    }
}

fn main() {
    assert_eq!(
        Solution::range_sum_bst(Some(Rc::new(RefCell::new(TreeNode::new(2)))), 1, 10),
        2
    );
}
