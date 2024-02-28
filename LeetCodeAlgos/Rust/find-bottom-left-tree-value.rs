// Given the root of a binary tree, return the leftmost value in the last row of the tree.

// Constraints:
// The number of nodes in the tree is in the range [1, 104].
// -231 <= Node.val <= 231 - 1

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

struct Solution {}
impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return -1;
        }
        let root = root.unwrap();

        return Self::manage_levels(&mut vec![root])
    }

    fn manage_levels(curr: &mut Vec<Rc<RefCell<TreeNode>>>) ->i32 {
        let mut new_level = Vec::new();
        for node in curr.iter() {
            let node = node.borrow();
            if let Some(left) = &node.left{
                new_level.push(left.clone());
            }
            if let Some(right) = &node.right {
                new_level.push(right.clone());
            }
        }
        if new_level.is_empty() {
            return curr[0].borrow().val;
        }
        return Self::manage_levels(&mut new_level);
    }

}

///! Leftmost overall, not bottom row left
// impl Solution {
//     pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         if root.is_none() {
//             return -1;
//         }
//         return Self::left_node(root.unwrap());
//     }

//     pub fn left_node(node: Rc<RefCell<TreeNode>>) -> i32 {
//         let curr = node.borrow();
//         match (curr.left.clone(), curr.right.clone()) {
//             (None, None) => curr.val,
//             (Some(left), Some(_)) | (Some(left), None) => Self::left_node(left),
//             (None, Some(right)) => Self::left_node(right),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_bottom_left_value() {
        assert_eq!(
            Solution::find_bottom_left_value(Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            })))),
            1
        );
    }

    #[test]
    fn test_find_bottom_left_value2() {
        assert_eq!(
            Solution::find_bottom_left_value(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(6))))
                }))),
            })))),
            7
        );
    }
}
