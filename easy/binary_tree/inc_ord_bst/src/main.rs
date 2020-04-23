// Given a binary search tree, rearrange the tree in in-order so that the leftmost node in the tree is now the root of the tree, and every node has no left child and only 1 right child.

// Example 1:
// Input: [5,3,6,2,4,null,8,1,null,null,null,7,9]

//        5
//       / \
//     3    6
//    / \    \
//   2   4    8
//  /        / \
// 1        7   9

// Output: [1,null,2,null,3,null,4,null,5,null,6,null,7,null,8,null,9]

//  1
//   \
//    2
//     \
//      3
//       \
//        4
//         \
//          5
//           \
//            6
//             \
//              7
//               \
//                8
//                 \
//                  9

// Constraints:

// The number of nodes in the given tree will be between 1 and 100.
// Each node will have a unique integer value from 0 to 1000.

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
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        println!("{:?}", root);
        let values = Self::get_values(root);
        println!("{:?}", values);

        let mut new_tree: Option<Rc<RefCell<TreeNode>>> = None;
        for num in values.iter().rev() {
            let node = Some(Rc::new(RefCell::new(TreeNode {
                val: *num,
                left: None,
                right: new_tree.clone(),
            })));
            new_tree = node;
        }
        new_tree
    }

    fn get_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        Self::inorder(root, &mut result);

        result
    }

    fn inorder(node: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
        if node.is_none() {
            return;
        }
        let _node = node.unwrap();
        let _node = _node.borrow();

        Self::inorder(_node.left.clone(), values);

        values.push(_node.val);

        Self::inorder(_node.right.clone(), values);
    }
}

fn main() {
    assert_eq!(
        Solution::increasing_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                })))
            })))
        })))),
        None
    );
}
