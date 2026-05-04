// There is an undirected tree with n nodes labeled from 0 to n - 1, and rooted at node 0. You are given a 2D integer array edges of length n - 1, where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the tree.

// A node is good if all the subtrees rooted at its children have the same size.

// Return the number of good nodes in the given tree.

// A subtree of treeName is a tree consisting of a node in treeName and all of its descendants.

// Constraints:
// - 2 <= n <= 105
// - edges.length == n - 1
// - edges[i].length == 2
// - 0 <= ai, bi < n
// - The input is generated such that edges represents a valid tree.

struct Solution;
impl Solution {
    // ❌ Solution makes the assumption that the input edges are given in a parent-child order, which is not guaranteed by the problem statement.
    pub fn _count_good_nodes1(mut edges: Vec<Vec<i32>>) -> i32 {
        for edge in &mut edges {
            edge.sort_unstable();
        }
        edges.sort_unstable_by_key(|e| (-e[1], -e[0]));

        let mut ans = 0;
        let mut nodes = vec![(0i32, Vec::<usize>::new()); edges.len() + 1];

        for edge in edges {
            // Add bottom idx to top's must check vec
            let bottom = edge[1] as usize;
            nodes[edge[0] as usize].1.push(bottom);
            // Update the bottom's weight
            nodes[bottom].0 += 1;
            // If bottom_node has no connected lower nodes, it's Good
            if nodes[bottom].1.is_empty() {
                ans += 1;
                continue;
            }

            // If bottom_node has connected lower node:
            // - add lower node weight to current node weight
            let expected = nodes[nodes[bottom].1[0]].0;
            nodes[bottom].0 += expected;
            // - make sure each lower node has equal weight
            let mut all = true;
            for i in 1..nodes[bottom].1.len() {
                let weight = nodes[nodes[bottom].1[i]].0;
                nodes[bottom].0 += weight;
                if weight != expected {
                    all = false;
                }
            }
            if all {
                // All same weight make current Good
                ans += 1;
            }
        }

        let expected = nodes[nodes[0].1[0]].0;
        let mut all = true;
        for i in 1..nodes[0].1.len() {
            let weight = nodes[nodes[0].1[i]].0;
            if weight != expected {
                all = false;
            }
        }
        if all {
            // All same weight make current Good
            ans += 1;
        }

        ans
    }

    pub fn count_good_nodes(edges: Vec<Vec<i32>>) -> i32 {
        let node_count = edges.len() + 1;
        let mut adjacency = vec![Vec::<usize>::new(); node_count];

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adjacency[u].push(v);
            adjacency[v].push(u);
        }

        let mut parent_of = vec![usize::MAX; node_count];
        let mut dfs_order = Vec::with_capacity(node_count);
        let mut node_stack = vec![0usize];
        parent_of[0] = 0;

        while let Some(node) = node_stack.pop() {
            dfs_order.push(node);
            for &neighbor in &adjacency[node] {
                if neighbor == parent_of[node] {
                    continue;
                }
                parent_of[neighbor] = node;
                node_stack.push(neighbor);
            }
        }

        let mut subtree_size = vec![1i32; node_count];
        let mut good_node_count = 0;

        for &node in dfs_order.iter().rev() {
            let mut child_subtree_sizes = Vec::new();

            for &neighbor in &adjacency[node] {
                if neighbor == parent_of[node] {
                    continue;
                }
                child_subtree_sizes.push(subtree_size[neighbor]);
                subtree_size[node] += subtree_size[neighbor];
            }

            // A node is good when every child subtree has the same size.
            if child_subtree_sizes.len() <= 1
                || child_subtree_sizes
                    .windows(2)
                    .all(|pair| pair[0] == pair[1])
            {
                good_node_count += 1;
            }
        }

        good_node_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 5],
            vec![2, 6],
        ];
        assert_eq!(Solution::count_good_nodes(edges), 7);
    }

    #[test]
    fn test2() {
        let edges = vec![
            vec![0, 1],
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![0, 5],
            vec![1, 6],
            vec![2, 7],
            vec![3, 8],
        ];
        assert_eq!(Solution::count_good_nodes(edges), 6);
    }

    #[test]
    fn test3() {
        let edges = vec![
            vec![0, 1],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![0, 5],
            vec![5, 6],
            vec![6, 7],
            vec![7, 8],
            vec![0, 9],
            vec![9, 10],
            vec![9, 12],
            vec![10, 11],
        ];
        assert_eq!(Solution::count_good_nodes(edges), 12);
    }
}
