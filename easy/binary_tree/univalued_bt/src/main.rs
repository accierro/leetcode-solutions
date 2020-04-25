// A binary tree is univalued if every node in the tree has the same value.

// Return true if and only if the given tree is univalued.

// Example 1:

// Input: [1,1,1,1,1,null,1]
// Output: true
// Example 2:

// Input: [2,2,2,5,2]
// Output: false

// Note:

// The number of nodes in the given tree will be in the range [1, 100].
// Each node's value will be an integer in the range [0, 99]

struct Solution;

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
impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return false;
        }
        let _root = root.unwrap();
        let val = _root.borrow().val;
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![Rc::clone(&_root)];

        while stack.len() > 0 {
            let node = stack.pop().unwrap();

            if node.borrow().val != val {
                return false;
            }

            if node.borrow().left.is_some() {
                let next_node = node.borrow();
                stack.push(Rc::clone(&next_node.left.as_ref().unwrap()));
            }

            if node.borrow().right.is_some() {
                let next_node = node.borrow();
                stack.push(Rc::clone(&next_node.right.as_ref().unwrap()));
            }
        }

        true
    }
}

fn main() {
    assert_eq!(
        Solution::is_unival_tree(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })))),
        true
    );
}
