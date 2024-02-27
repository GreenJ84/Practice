// Given the roots of two binary trees p and q, write a function to check if they are the same or not.

// Two binary trees are considered the same if they are structurally identical, and the nodes have the same value.

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq, Clone)]
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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (None, _) => false,
            (_, None) => false,
            (Some(p), Some(q)) => {
                if p.borrow().val != q.borrow().val {
                    return false;
                }
                return Self::is_same_tree(p.borrow().left.clone(), q.borrow().left.clone())
                    && Self::is_same_tree(p.borrow().right.clone(), q.borrow().right.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_same_tree() {
        assert_eq!(
            Solution::is_same_tree(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                }))),
            ),
            true
        );
    }

    #[test]
    fn test_is_same_tree2() {
        assert_eq!(
            Solution::is_same_tree(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    right: None,
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                }))),
            ),
            false
        );
    }

    #[test]
    fn test_is_same_tree3() {
        assert_eq!(
            Solution::is_same_tree(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                }))),
            ),
            false
        );
    }
}
