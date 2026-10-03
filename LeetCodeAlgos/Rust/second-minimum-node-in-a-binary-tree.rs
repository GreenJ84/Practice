// Given a non-empty special binary tree consisting of nodes with the non-negative value, where each node in this tree has exactly two or zero sub-node. If the node has two sub-nodes, then this node's value is the smaller value among its two sub-nodes. More formally, the property root.val = min(root.left.val, root.right.val) always holds.

// Given such a binary tree, you need to output the second minimum value in the set made of all the nodes' value in the whole tree.

// If no such second minimum value exists, output -1 instead.

// Constraints:
// The number of nodes in the tree is in the range [1, 25].
// 1 <= Node.val <= 2^31 - 1
// root.val == min(root.left.val, root.right.val) for each internal node of the tree.

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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return -1;
        }
        let root = root.as_ref().unwrap();
        let min = root.borrow().val;
        let mut max = None;

        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::from([Rc::clone(root)]);
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            let node = node.borrow();
            match max {
                None => {
                    if node.val > min {
                        max = Some(node.val);
                    }
                }
                Some(m) => {
                    if node.val > min && node.val < m {
                        max = Some(node.val);
                    } else if node.val >= m {
                        continue;
                    }
                }
            }

            if node.left.is_some() {
                queue.push_back(Rc::clone(node.left.as_ref().unwrap()));
                queue.push_back(Rc::clone(node.right.as_ref().unwrap()));
            }
        }
        return if max.is_none() { -1 } else { max.unwrap() };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(Solution::find_second_minimum_value(root), 5);
    }

    #[test]
    fn test_2() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        })));
        assert_eq!(Solution::find_second_minimum_value(root), -1);
    }
}
