// Given the root of a Binary Search Tree (BST), return the minimum difference between the values of any two different nodes in the tree.

// Constraints:
// The number of nodes in the tree is in the range [2, 100].
// 0 <= Node.val <= 10^5

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
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root.as_ref() {
            Some(node) => Self::min_diff(node, i32::MAX, None, None),
            None => -1,
        }
    }

    fn min_diff(
        node: &Rc<RefCell<TreeNode>>,
        mut min_val: i32,
        min: Option<i32>,
        max: Option<i32>,
    ) -> i32 {
        let node = node.borrow();
        // Distance check
        if let Some(m) = min {
            min_val = min_val.min((node.val - m).abs());
        }
        if let Some(m) = max {
            min_val = min_val.min((node.val - m).abs());
        }
        // Traversal
        if let Some(left) = node.left.as_ref() {
            min_val = min_val.min(Self::min_diff(&left, min_val, min, Some(node.val)));
        }
        if let Some(right) = node.right.as_ref() {
            min_val = min_val.min(Self::min_diff(&right, min_val, Some(node.val), max));
        }
        min_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
        })));
        assert_eq!(Solution::min_diff_in_bst(root), 1);
    }

    #[test]
    fn test21() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 48,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 12,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 49,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(Solution::min_diff_in_bst(root), 1);
    }
}
