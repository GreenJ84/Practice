// You are given the root of a binary tree that consists of exactly 3 nodes: the root, its left child, and its right child.

// Return true if the value of the root is equal to the sum of the values of its two children, or false otherwise.

// Constraints:
// The tree consists only of the root, its left child, and its right child.
// -100 <= Node.val <= 100

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
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root = root.as_ref().unwrap().borrow();

        let mut child_sum = root.left.as_ref().unwrap().borrow().val;
        child_sum += root.right.as_ref().unwrap().borrow().val;

        root.val == child_sum
    }

    // Voiding explicit constraints for safety
    pub fn check_tree1(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root.as_ref() {
            Some(node) => {
                let node = node.borrow();
                let mut child_sum = 0;
                if let Some(left) = node.left.as_ref() {
                    child_sum += left.borrow().val;
                }
                if let Some(right) = node.right.as_ref() {
                    child_sum += right.borrow().val;
                }
                node.val == child_sum
            }
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
        })));
        assert_eq!(Solution::check_tree(root), true);
    }

    #[test]
    fn test_2() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
        })));
        assert_eq!(Solution::check_tree(root), false);
    }
}
