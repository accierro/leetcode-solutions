// Given the root node of a binary search tree (BST) and a value. You need to find the node in the BST that the node's value equals the given value. Return the subtree rooted with that node. If such node doesn't exist, you should return NULL.

// For example,

// Given the tree:
//         4
//        / \
//       2   7
//      / \
//     1   3

// And the value to search: 2
// You should return this subtree:

//       2
//      / \
//     1   3
// In the example above, if we want to search the value 5, since there is no node with value 5, we should return NULL.

// Note that an empty tree is represented by NULL, therefore you would see the expected output (serialized tree format) as [], not null.

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
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let _root = root.unwrap();
        let _val = _root.borrow().val;
        println!("{}", _val);
        if _val == val {
            return Some(_root);
        }
        if _val > val {
            Self::search_bst(_root.borrow().left.clone(), val)
        } else {
            Self::search_bst(_root.borrow().right.clone(), val)
        }
    }
}

// More elegant way
// impl Solution {
//     pub fn search_bst(
//         root: Option<Rc<RefCell<TreeNode>>>,
//         val: i32,
//     ) -> Option<Rc<RefCell<TreeNode>>> {
//         if let Some(v) = &root {
//             if v.borrow().val > val {
//                 return Self::search_bst(v.borrow().left.clone(), val);
//             } else if v.borrow().val < val {
//                 return Self::search_bst(v.borrow().right.clone(), val);
//             }
//         }

//         return root;
//     }
// }

fn main() {
    println!("HELLO")
}
