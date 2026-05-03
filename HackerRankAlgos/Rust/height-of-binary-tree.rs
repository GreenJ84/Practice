// Given the root of a binary search tree, return the height of the tree. Height is the number of nodes along the longest path from root to leaf.

// Constraints
// - 0 <= n <= 1000
// - values.length == n
// - left_child.length == n
// - right_child.length == n
// - For all 0 <= i < n: -10^9 <= values[i] <= 10^9
// - All values[i] are unique
// - For all 0 <= i < n: left_child[i] == -1 or (0 <= left_child[i] < n)
// - For all 0 <= i < n: right_child[i] == -1 or (0 <= right_child[i] < n)
// - For all 0 <= i < n: if left_child[i] != -1 then values[left_child[i]] < values[i]
// - For all 0 <= i < n: if right_child[i] != -1 then values[right_child[i]] > values[i]
// - The set of edges defined by (i, left_child[i]) and (i, right_child[i]) forms a single connected acyclic graph (a tree)

// fn getBinarySearchTreeHeight(values: &[i32], left_child: &[i32], right_child: &[i32]) -> i32 {
fn get_binary_search_tree_height(values: &[i32], left_child: &[i32], right_child: &[i32]) -> i32 {
    if values.is_empty() {
      return 0;
    }
    let mut stack = vec![(0usize, 1)];
    let mut max_height = 1;
    while let Some((idx, height)) = stack.pop() {
      if left_child[idx] != -1 {
        stack.push((left_child[idx] as usize, height + 1));
      }
      if right_child[idx] != -1 {
        stack.push((right_child[idx] as usize, height + 1));
      }
      max_height = max_height.max(height);
    }
    max_height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let values = [4, 2, 6, 1, 3, 5, 7];
        let left_child = [1, 3, 5, -1, -1, -1, -1];
        let right_child = [2, 4, 6, -1, -1, -1, -1];
        assert_eq!(get_binary_search_tree_height(&values, &left_child, &right_child), 3);
    }
}
