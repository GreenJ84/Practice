// There is an undirected graph consisting of n nodes numbered from 0 to n - 1. You are given a 0-indexed integer array vals of length n where vals[i] denotes the value of the ith node.
//
// You are also given a 2D integer array edges where edges[i] = [ai, bi] denotes that there exists an undirected edge connecting nodes ai and bi.
//
// A star graph is a subgraph of the given graph having a center node containing 0 or more neighbors. In other words, it is a subset of edges of the given graph such that there exists a common node for all edges.

use std::collections::*;
use std::cmp::{Reverse, Ordering};

#[derive(Clone, Debug)]
struct Node {
    pub val: i32,
    pub connections: BinaryHeap<Reverse<i32>>
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}
impl Eq for Node {}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.val.partial_cmp(&other.val)
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering{
        other.val.cmp(&self.val)
    }
}

impl Node{
    pub fn new(val: i32) -> Node{
        Node {
            val,
            connections: BinaryHeap::new()
        }
    }
}

struct Solution {}
impl Solution {
    fn manage_heap_push(node: &mut Node, val: i32, k: i32) {
        if let Some(&Reverse(smallest)) = node.connections.peek() {
            if node.connections.len() < k as usize {
                if smallest > 0 && val > 0 {
                    node.connections.push(Reverse(val));
                } else if smallest < 0 && val > smallest {
                    node.connections.pop();
                    Self::manage_heap_push(node, val, k);
                }
            } else if val > smallest {
                node.connections.pop();
                node.connections.push(Reverse(val));
            }
        } else if k > 0{
            node.connections.push(Reverse(val));
        }
    }

    pub fn max_star_sum(vals: Vec<i32>, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut max_sum = *vals.iter().max().unwrap();
        if edges.is_empty() { return max_sum; }

        let mut nodes: HashMap<i32, Node> = HashMap::new();
        for edge in &edges{
            let mut a_node = nodes.entry(edge[0])
                .or_insert( Node::new(vals[edge[0] as usize]) );
            let a_val: i32 = vals[edge[1] as usize];
            Self::manage_heap_push(&mut a_node, a_val, k);

            let mut b_node = nodes.entry(edge[1])
                .or_insert( Node::new(vals[edge[1] as usize]) );
            let b_val: i32 = vals[edge[0] as usize];
            Self::manage_heap_push(&mut b_node, b_val, k);
        }
        for (_position, node) in nodes.iter(){
            max_sum = max_sum.max(node.val + (*node).connections.iter().map(|conn| conn.0).sum::<i32>());
        }
        max_sum
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        assert_eq!(
            Solution::max_star_sum(
                Vec::from([1,2,3,4,10,-10,-20]),
                Vec::from([vec![0,1], vec![1,2], vec![1,3], vec![3,4], vec![3,5], vec![3,6]] ),
                2
            ),
            16
        );
    }

    #[test]
    fn test2(){
        assert_eq!(
            Solution::max_star_sum(
                Vec::from([-5]),
                Vec::from([]),
                0
            ),
            -5
        );
    }

    #[test]
    fn test3(){
        assert_eq!(
            Solution::max_star_sum(
                Vec::from([1,2,3,4,10,-10,-20, 20]),
                Vec::from([vec![0,1], vec![1,2], vec![1,3], vec![3,4], vec![3,5], vec![3,6]] ),
                2
            ),
            20
        );
    }
}