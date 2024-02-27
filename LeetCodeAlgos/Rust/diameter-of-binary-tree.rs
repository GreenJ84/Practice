// Given the root of a binary tree, return the length of the diameter of the tree.
// The diameter of a binary tree is the length of the longest path between any two nodes in a tree. This path may or may not pass through the root.
// The length of a path between two nodes is represented by the number of edges between them.

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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut track = Vec::new();
        if let Some(root) = root {
            if root.borrow().left.is_none() && root.borrow().right.is_none() {
                return 0;
            }
            Self::dps(root, 0, &mut track);
            return *track.iter().max().unwrap();
        } else {
            return 0;
        }
    }

    fn dps(node: Rc<RefCell<TreeNode>>, dist: i32, track: &mut Vec<i32>) -> i32 {
        let b_node = node.borrow();
        let left = if b_node.left.is_some() { Self::dps(b_node.left.clone().unwrap(), 1, track) } else { 0 };
        let right = if b_node.right.is_some() {Self::dps(b_node.right.clone().unwrap(), 1, track)} else { 0 };
        track.push(left + right);
        return if left >= right {
            dist + left
        } else {
            dist + right
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diameter_of_binary_tree() {
        assert_eq!(
            Solution::diameter_of_binary_tree(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            })))),
            3
        );
    }

    #[test]
    fn test_diameter_of_binary_tree2() {
        assert_eq!(
            Solution::diameter_of_binary_tree(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: None,
            })))),
            1
        );
    }

    #[test]
    fn test_diameter_of_binary_tree3() {
        assert_eq!(Solution::diameter_of_binary_tree(None), 0);
    }

    #[test]
    fn test_diameter_of_binary_tree4() {
        assert_eq!(
            Solution::diameter_of_binary_tree(Some(Rc::new(RefCell::new(TreeNode::new(1))))),
            0
        );
    }
}
