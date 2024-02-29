// A binary tree is named Even-Odd if it meets the following conditions:

// The root of the binary tree is at level index 0, its children are at level index 1, their children are at level index 2, etc.
// For every even-indexed level, all nodes at the level have odd integer values in strictly increasing order (from left to right).
// For every odd-indexed level, all nodes at the level have even integer values in strictly decreasing order (from left to right).
// Given the root of a binary tree, return true if the binary tree is Even-Odd, otherwise return false.

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
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            if root.borrow().val % 2 != 1 {
                return false;
            }
            let mut stack = vec![root];
            return Self::iterate_tree(0, &mut stack);
        }
        return false;
    }

    fn iterate_tree(level: usize, nodes: &mut Vec<Rc<RefCell<TreeNode>>>) -> bool {
        let current_level = level + 1;
        println!("level: {}", current_level);

        let mut next_level = Vec::new();
        let mut prev_value;
        let is_odd;
        if current_level % 2 == 0 {
            prev_value = 0;
            is_odd = true;
        } else {
            prev_value = i32::MAX;
            is_odd = false;
        }

        for node in nodes.drain(..) {
            let node = node.borrow();

            for side in vec![node.left.clone(), node.right.clone()].iter() {
                match side {
                    None => continue,
                    Some(side) => {
                        println!("side: {}, odd: {}, prev: {}", side.borrow().val, is_odd, prev_value);

                        if (side.borrow().val % 2 == 1) != is_odd {
                            return false;
                        }
                        match is_odd{
                            true => {
                                if side.borrow().val <= prev_value {
                                    return false;
                                }
                            }
                            false => {
                                if side.borrow().val >= prev_value {
                                    return false;
                                }
                            }
                        }
                        prev_value = side.borrow().val;
                        next_level.push(side.clone());
                    }
                }
            }
        }
        return if next_level.is_empty() {
            true
        } else {
            Self::iterate_tree(current_level, &mut next_level)
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even_odd_tree1() {
        assert_eq!(
            Solution::is_even_odd_tree(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 10,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(12)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
                    }))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 9,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    }))),
                }))),
            })))),
            true
        )
    }

    #[test]
    fn test_is_even_odd_tree2() {
        assert_eq!(
            Solution::is_even_odd_tree(Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    right: None,
                }))),
            })))),
            false
        )
    }

    #[test]
    fn test_is_even_odd_tree3() {
        assert_eq!(
            Solution::is_even_odd_tree(Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    right: None,
                }))),
            })))),
            false
        )
    }
}
