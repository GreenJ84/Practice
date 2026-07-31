// Given the root of a binary tree with unique values and the values of two different nodes of the tree x and y, return true if the nodes corresponding to the values x and y in the tree are cousins, or false otherwise.

// Two nodes of a binary tree are cousins if they have the same depth with different parents.

// Note that in a binary tree, the root node is at the depth 0, and children of each depth k node are at the depth k + 1.

// Constraints:
// The number of nodes in the tree is in the range [2, 100].
// 1 <= Node.val <= 100
// Each node has a unique value.
// x != y
// x and y are exist in the tree.

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

#[derive(Debug)]
struct Pointer(Option<i32>, Option<i32>);
impl Pointer {
    fn is_empty(&self) -> bool {
        self.0.is_none() && self.1.is_none()
    }
}

struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let Some(root) = root else {
            return false;
        };
        let mut queue = std::collections::VecDeque::from([root]);
        while !queue.is_empty() {
            let n = queue.len();
            let mut x_parent = None;
            let mut y_parent = None;

            for _ in 0..n {
                let cur = queue.pop_front().unwrap();
                let node = cur.borrow();
                if let Some(left) = &node.left {
                    let val = left.borrow().val;
                    if val == x {
                        x_parent = Some(node.val);
                    }
                    if val == y {
                        y_parent = Some(node.val);
                    }
                    queue.push_back(Rc::clone(left));
                }

                if let Some(right) = &node.right {
                    let val = right.borrow().val;
                    if val == x {
                        x_parent = Some(node.val);
                    }
                    if val == y {
                        y_parent = Some(node.val);
                    }
                    queue.push_back(Rc::clone(right));
                }
            }
            match (x_parent, y_parent) {
                (Some(xp), Some(yp)) => {
                    return xp != yp;
                }
                (None, None) => {}
                _ => {
                    return false;
                }
            }
        }
        false
    }

    pub fn is_cousins1(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let mut x_ptr = Pointer(None, None); // parent_val, depth
        let mut y_ptr = Pointer(None, None);

        let mut queue = std::collections::VecDeque::new();
        queue.push_back((root.unwrap(), 0));
        while (x_ptr.is_empty() || y_ptr.is_empty()) && !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            let tree = node.0.borrow();
            if let Some(node_ref) = &tree.left {
                let val = node_ref.borrow().val;
                if val == x {
                    x_ptr = Pointer(Some(tree.val), Some(node.1 + 1));
                } else if val == y {
                    y_ptr = Pointer(Some(tree.val), Some(node.1 + 1));
                }
                queue.push_back((node_ref.clone(), node.1 + 1))
            }

            if let Some(node_ref) = &tree.right {
                let val = node_ref.borrow().val;
                if val == x {
                    x_ptr = Pointer(Some(tree.val), Some(node.1 + 1));
                } else if val == y {
                    y_ptr = Pointer(Some(tree.val), Some(node.1 + 1));
                }
                queue.push_back((node_ref.clone(), node.1 + 1))
            }
        }
        x_ptr.0 != y_ptr.0 && x_ptr.1 == y_ptr.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        assert_eq!(Solution::is_cousins(root, 4, 3), false);
    }

    #[test]
    fn test_2() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
        })));
        assert_eq!(Solution::is_cousins(root, 5, 4), true);
    }

    #[test]
    fn test_3() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        assert_eq!(Solution::is_cousins(root, 2, 3), false);
    }
}
