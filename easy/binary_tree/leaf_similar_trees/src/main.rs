// Consider all the leaves of a binary tree.  From left to right order, the values of those leaves form a leaf value sequence.

// For example, in the given tree above, the leaf value sequence is (6, 7, 4, 9, 8).

// Two binary trees are considered leaf-similar if their leaf value sequence is the same.

// Return true if and only if the two given trees with head nodes root1 and root2 are leaf-similar.

// Constraints:

// Both of the given trees will have between 1 and 200 nodes.
// Both of the given trees will have values between 0 and 200

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
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let res1 = Self::leaf_values(root1.clone());
        let res2 = Self::leaf_values(root2.clone());

        if res1.len() != res2.len() {
            return false;
        }

        res1 == res2
    }

    pub fn leaf_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        if root.is_some() {
            stack.push(root);
            while stack.len() > 0 {
                let node = stack.pop().unwrap().unwrap();
                let val = node.borrow().val;
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    result.push(val);
                }

                if node.borrow().left.is_some() {
                    let left = node.borrow().left.clone();
                    stack.push(left);
                }

                if node.borrow().right.is_some() {
                    let right = node.borrow().right.clone();
                    stack.push(right);
                }
            }
        }

        result
    }
}

fn main() {
    let check = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: None,
                right: None,
            }))),
        }))),
    })));
    let same = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: None,
                right: None,
            }))),
        }))),
    })));
    let different = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
        }))),
    })));
    assert_eq!(Solution::leaf_similar(check.clone(), same), true);
    assert_eq!(Solution::leaf_similar(check.clone(), different), false);
}
