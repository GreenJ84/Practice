// You are given the root of a binary tree and a positive integer k.

// The level sum in the tree is the sum of the values of the nodes that are on the same level.

// Return the kth largest level sum in the tree (not necessarily distinct). If there are fewer than k levels in the tree, return -1.

// Note that two nodes are on the same level if they have the same distance from the root.

// Constraints:

// - The number of nodes in the tree is n.
// - 2 <= n <= 105
// - 1 <= Node.val <= 106
// - 1 <= k <= n

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
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let mut heap = std::collections::BinaryHeap::with_capacity(k as usize);

        let mut lvl = vec![root.unwrap()]; // Vec<Rc<RefCell<TreeNode>>>
        let mut lvl_sum = 0i64;
        let mut nxt = Vec::new();
        while !lvl.is_empty() {
            for n in &lvl { // n = Rc<RefCell<TreeNode>>
                let node = n.borrow();
                lvl_sum += node.val as i64;
                if node.left.is_some() {
                    nxt.push(node.left.clone().unwrap());
                }
                if node.right.is_some() {
                    nxt.push(node.right.clone().unwrap());
                }
            }
            heap.push(lvl_sum);
            lvl_sum = 0;
            lvl = nxt;
            nxt = Vec::new();
        }
        if heap.len() < k as usize {
            return -1;
        }
        for _ in 1..k {
            heap.pop();
        }
        heap.pop().unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree_from_vec(vec: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let nodes = vec.iter().map(|&val| Rc::new(RefCell::new(TreeNode::new(val)))).collect::<Vec<_>>();
        for i in 0..nodes.len() {
            let left_idx = 2 * i + 1;
            let right_idx = 2 * i + 2;
            if left_idx < nodes.len() {
                nodes[i].borrow_mut().left = Some(nodes[left_idx].clone());
            }
            if right_idx < nodes.len() {
                nodes[i].borrow_mut().right = Some(nodes[right_idx].clone());
            }
        }

        nodes.get(0).cloned()
    }

    #[test]
    fn test_1() {
        let root = build_tree_from_vec(vec![5, 8, 9, 2, 1, 3, 7, 4, 6]);
        let k = 2;
        assert_eq!(Solution::kth_largest_level_sum(root, k), 13);
    }

    #[test]
    fn test_2() {
        let root = build_tree_from_vec(vec![1, 2, 3, 4, 5, 6, 7]);
        let k = 1;
        assert_eq!(Solution::kth_largest_level_sum(root, k), 22);
    }

    #[test]
    fn test_3() {
        let root = build_tree_from_vec(vec![1, 2, 3, 4, 5, 6, 7]);
        let k = 2;
        assert_eq!(Solution::kth_largest_level_sum(root, k), 5);
    }
}

// Example 1:

// Input: root = [5,8,9,2,1,3,7,4,6], k = 2
// Output: 13
// Explanation: The level sums are the following:
// - Level 1: 5.
// - Level 2: 8 + 9 = 17.
// - Level 3: 2 + 1 + 3 + 7 = 13.
// - Level 4: 4 + 6 = 10.
// The 2nd largest level sum is 13.
// Example 2:

// Input: root = [1,2,null,3], k = 1
// Output: 3
// Explanation: The largest level sum is 3.
