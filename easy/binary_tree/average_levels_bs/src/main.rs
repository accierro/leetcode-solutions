// Given a non-empty binary tree, return the average value of the nodes on each level in the form of an array.
// Example 1:
// Input:
//     3
//    / \
//   9  20
//     /  \
//    15   7
// Output: [3, 14.5, 11]
// Explanation:
// The average value of nodes on level 0 is 3,  on level 1 is 14.5, and on level 2 is 11. Hence return [3, 14.5, 11].
// Note:
// The range of node's value is in the range of 32-bit signed integer.

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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut levels_num: Vec<(i64, i64)> = Vec::new();

        Self::h(&root, 0, &mut levels_num);
        levels_num.iter().map(|e| e.0 as f64 / e.1 as f64).collect()
    }

    fn h(root: &Option<Rc<RefCell<TreeNode>>>, level: usize, arr: &mut Vec<(i64, i64)>) {
        if let Some(v) = root {
            let node = v.borrow();

            if let Some(pos) = arr.get_mut(level as usize) {
                pos.0 += node.val as i64;
                pos.1 += 1;
            } else {
                arr.push((node.val as i64, 1));
            }

            if node.left.is_some() {
                Self::h(&node.left, level + 1, arr);
            }

            if node.right.is_some() {
                Self::h(&node.right, level + 1, arr);
            }
        }
    }
}

fn main() {
    assert_eq!(
        Solution::average_of_levels(Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None
                })))
            })))
        })))),
        vec![3.0, 14.5, 11.0]
    );
    assert_eq!(
        Solution::average_of_levels(Some(Rc::new(RefCell::new(TreeNode {
            val: 2147483647,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2147483647,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2147483647,
                left: None,
                right: None
            })))
        })))),
        vec![2147483647.0, 2147483647.0]
    );
}
