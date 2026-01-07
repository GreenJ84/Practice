// Given the root of a binary tree, the level of its root is 1, the level of its children is 2, and so on.

// Return the smallest level x such that the sum of all the values of nodes at level x is maximal.

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
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut max = (0i32, i32::MIN);

        let mut lvl = (1i32, vec![root.unwrap()]);
        while !lvl.1.is_empty() {
            let mut level_sum = 0i32;
            let mut next_queue = vec![];

            for node in lvl.1 {
                let node_ref = node.borrow();
                level_sum += node_ref.val;

                if let Some(left) = &node_ref.left {
                    next_queue.push(Rc::clone(left));
                }
                if let Some(right) = &node_ref.right {
                    next_queue.push(Rc::clone(right));
                }
            }

            if level_sum > max.1 {
                max = (lvl.0, level_sum);
            }

            lvl = (lvl.0 + 1, next_queue);
        }
        max.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_tree(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    #[test]
    fn test_example_1() {
        let root = create_tree(
            1,
            create_tree(7, create_tree(7, None, None), create_tree(-8, None, None)),
            create_tree(0, None, None),
        );
        assert_eq!(Solution::max_level_sum(root), 2);
    }

    #[test]
    fn test_example_2() {
        let root = create_tree(
            989,
            None,
            create_tree(
                10250,
                create_tree(98693, None, None),
                create_tree(-89388, None, None),
            ),
        );
        assert_eq!(Solution::max_level_sum(root), 2);
    }

    #[test]
    fn test_single_node() {
        let root = create_tree(5, None, None);
        assert_eq!(Solution::max_level_sum(root), 1);
    }

    #[test]
    fn test_all_negative_values() {
        let root = create_tree(
            -1,
            create_tree(-2, create_tree(-3, None, None), create_tree(-4, None, None)),
            create_tree(-5, None, None),
        );
        assert_eq!(Solution::max_level_sum(root), 1);
    }

    #[test]
    fn test_max_at_last_level() {
        let root = create_tree(1, create_tree(1, create_tree(10, None, None), None), None);
        assert_eq!(Solution::max_level_sum(root), 3);
    }
}
