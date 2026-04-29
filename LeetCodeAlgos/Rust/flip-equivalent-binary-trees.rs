// For a binary tree T, we can define a flip operation as follows: choose any node, and swap the left and right child subtrees.

// A binary tree X is flip equivalent to a binary tree Y if and only if we can make X equal to Y after some number of flip operations.

// Given the roots of two binary trees root1 and root2, return true if the two trees are flip equivalent or false otherwise.

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
    pub fn build_tree_from_vec(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        use std::collections::VecDeque;

        let mut iter = vec.into_iter();
        let root_val = match iter.next() {
            Some(v) => v,
            None => return None,
        };
        let root_val = match root_val {
            Some(v) => v,
            None => return None,
        };

        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        queue.push_back(root.clone());

        while let Some(node) = queue.pop_front() {
            if let Some(next) = iter.next() {
                if let Some(v) = next {
                    let left = Rc::new(RefCell::new(TreeNode::new(v)));
                    node.borrow_mut().left = Some(left.clone());
                    queue.push_back(left);
                }
            } else {
                break;
            }

            if let Some(next) = iter.next() {
                if let Some(v) = next {
                    let right = Rc::new(RefCell::new(TreeNode::new(v)));
                    node.borrow_mut().right = Some(right.clone());
                    queue.push_back(right);
                }
            } else {
                break;
            }
        }

        Some(root)
    }
}
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root1.is_none() && root2.is_none() {
            return true;
        } else if (root1.is_none() ^ root2.is_none())
            || root1.as_ref().map(|n| n.borrow().val) != root2.as_ref().map(|n| n.borrow().val)
        {
            return false;
        }

        let (mut lvl1, mut nxt1) = (vec![root1.unwrap()], vec![]);
        let (mut lvl2, mut nxt2) = (vec![root2.unwrap()], vec![]);
        while !lvl1.is_empty() && !lvl2.is_empty() {
            for i in 0..lvl1.len() {
                let (l1, r1) = (
                    lvl1[i].borrow().left.clone(),
                    lvl1[i].borrow().right.clone(),
                );
                let (l2, r2) = (
                    lvl2[i].borrow().left.clone(),
                    lvl2[i].borrow().right.clone(),
                );
                match (l1, r1, l2, r2) {
                    (l1, r1, l2, r2)
                        if l1.as_ref().map(|n| n.borrow().val)
                            == l2.as_ref().map(|n| n.borrow().val)
                            && r1.as_ref().map(|n| n.borrow().val)
                                == r2.as_ref().map(|n| n.borrow().val) =>
                    {
                        if let Some(left1) = l1 {
                            nxt1.push(left1);
                            nxt2.push(l2.unwrap());
                        }
                        if let Some(right1) = r1 {
                            nxt1.push(right1);
                            nxt2.push(r2.unwrap());
                        }
                    }
                    (l1, r1, l2, r2)
                        if l1.as_ref().map(|n| n.borrow().val)
                            == r2.as_ref().map(|n| n.borrow().val)
                            && r1.as_ref().map(|n| n.borrow().val)
                                == l2.as_ref().map(|n| n.borrow().val) =>
                    {
                        if let Some(left1) = l1 {
                            nxt1.push(left1);
                            nxt2.push(r2.unwrap());
                        }
                        if let Some(right1) = r1 {
                            nxt1.push(right1);
                            nxt2.push(l2.unwrap());
                        }
                    }
                    (_, _, _, _) => {
                        return false;
                    }
                }
            }
            lvl1 = nxt1.to_owned();
            nxt1 = vec![];
            lvl2 = nxt2.to_owned();
            nxt2 = vec![];
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let root1 = TreeNode::build_tree_from_vec(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            None,
            None,
            None,
            Some(7),
            Some(8),
        ]);
        let root2 = TreeNode::build_tree_from_vec(vec![
            Some(1),
            Some(3),
            Some(2),
            None,
            Some(6),
            Some(4),
            Some(5),
            None,
            None,
            None,
            None,
            Some(8),
            Some(7),
        ]);
        assert_eq!(Solution::flip_equiv(root1, root2), true);
    }

    #[test]
    fn test_2() {
        let root1 = TreeNode::build_tree_from_vec(vec![]);
        let root2 = TreeNode::build_tree_from_vec(vec![]);
        assert_eq!(Solution::flip_equiv(root1, root2), true);
    }

    #[test]
    fn test_3() {
        let root1 = TreeNode::build_tree_from_vec(vec![]);
        let root2 = TreeNode::build_tree_from_vec(vec![Some(1)]);
        assert_eq!(Solution::flip_equiv(root1, root2), false);
    }
}

// Example 1:

// Flipped Trees Diagram
// Input: root1 = [1,2,3,4,5,6,null,null,null,7,8], root2 = [1,3,2,null,6,4,5,null,null,null,null,8,7]
// Output: true
// Explanation: We flipped at nodes with values 1, 3, and 5.
// Example 2:

// Input: root1 = [], root2 = []
// Output: true
// Example 3:

// Input: root1 = [], root2 = [1]
// Output: false
