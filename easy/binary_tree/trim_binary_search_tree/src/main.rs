// Given a binary search tree and the lowest and highest boundaries as L and R, trim the tree so that all its elements lies in [L, R] (R >= L). You might need to change the root of the tree, so the result should return the new root of the trimmed binary search tree.

// Example 1:
// Input:
//     1
//    / \
//   0   2

//   L = 1
//   R = 2

// Output:
//     1
//       \
//        2
// Example 2:
// Input:
//     3
//    / \
//   0   4
//    \
//     2
//    /
//   1

//   L = 1
//   R = 3

// Output:
//       3
//      /
//    2
//   /
//  1

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
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        l: i32,
        r: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        let _root = root.unwrap();
        let mut node = _root.borrow_mut();

        if node.val < l {
            return Self::trim_bst(node.right.clone(), l, r);
        }
        if node.val > r {
            return Self::trim_bst(node.left.clone(), l, r);
        }

        node.left = Self::trim_bst(node.left.clone(), l, r);
        node.right = Self::trim_bst(node.right.clone(), l, r);

        drop(node);
        return Some(_root);
    }
}

fn main() {
    assert_eq!(
        Solution::trim_bst(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))),
            1,
            2
        ),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        })))
    )
}
