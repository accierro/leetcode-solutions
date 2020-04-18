// Given two binary trees and imagine that when you put one of them to cover the other, some nodes of the two trees are overlapped while the others are not.

// You need to merge them into a new binary tree. The merge rule is that if two nodes overlap, then sum node values up as the new value of the merged node. Otherwise, the NOT null node will be used as the node of new tree.

// Example 1:

// Input:
// 	Tree 1                     Tree 2
//           1                         2
//          / \                       / \
//         3   2                     1   3
//        /                           \   \
//       5                             4   7
// Output:
// Merged tree:
// 	     3
// 	    / \
// 	   4   5
// 	  / \   \
// 	 5   4   7

// Note: The merging process must start from the root nodes of both trees.

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
    pub fn merge_trees(
        t1: Option<Rc<RefCell<TreeNode>>>,
        t2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut current_node;
        let mut current_target;

        match &t1 {
            Some(x) => current_node = Rc::clone(&x),
            None => return t2,
        }

        match &t2 {
            Some(x) => current_target = (Rc::clone(&x)),
            None => return t1,
        }

        Self::merge(Rc::clone(&current_target), Rc::clone(&current_node));

        Some(current_target)
    }

    pub fn merge(target: Rc<RefCell<TreeNode>>, node: Rc<RefCell<TreeNode>>) {
        let mut target_node = target.borrow_mut();
        let mut node_node = node.borrow();
        target_node.val += node_node.val;

        if target_node.left.is_some() {
            if node_node.left.is_some() {
                Self::merge(
                    Rc::clone(&target_node.left.as_ref().unwrap()),
                    Rc::clone(&node_node.left.as_ref().unwrap()),
                )
            }
        } else {
            if node_node.left.is_some() {
                target_node.left = Some(Rc::clone(&node_node.left.as_ref().unwrap()));
            }
        }

        if target_node.right.is_some() {
            if node_node.right.is_some() {
                Self::merge(
                    Rc::clone(&target_node.right.as_ref().unwrap()),
                    Rc::clone(&node_node.right.as_ref().unwrap()),
                )
            }
        } else {
            if node_node.right.is_some() {
                target_node.right = Some(Rc::clone(&node_node.right.as_ref().unwrap()));
            }
        }
    }
}

fn main() {
    Solution::merge_trees(None, None);
}
