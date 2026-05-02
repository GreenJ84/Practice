// Given n computers labeled 0 to n-1 and a list of bidirectional communication links, find the number of connected components.

// Constraints
// - 1 <= n <= 1000
// - 0 <= links.length <= n * (n - 1) / 2
// - links[i].length == 2 for each valid i
// - 0 <= links[i][0] < n
// - 0 <= links[i][1] < n
// - links[i][0] != links[i][1] for each valid i (No self-loops)
// - Each pair [a, b] is bidirectional, i.e., [a, b] and [b, a] are considered the same link
// - No duplicate links (i.e., no repeated [a, b] or [b, a]) in links
// All values in each link are integers

use ::std::collections::HashSet;
// fn countIsolatedCommunicationGroups(links: &[Vec<i32>], n: i32) -> i32 {
fn count_isolated_communication_groups(links: &[Vec<i32>], n: i32) -> i32 {
    if links.is_empty() {
        return n;
    }

    let mut connections_map = vec![Vec::<i32>::new(); n as usize];
    for link in links {
      connections_map[link[0] as usize].push(link[1]);
      connections_map[link[1] as usize].push(link[0]);
    }

    let mut ans = 0i32;
    let mut seen = HashSet::<i32>::new();
    for node_idx in 0..n{
      if connections_map[node_idx as usize].is_empty() {
        continue;
      }
      ans += 1;
      
      let mut stack = Vec::<i32>::new();
      stack.push(node_idx);
      seen.insert(node_idx);

      while let Some(node) = stack.pop() {
        for connected in &connections_map[node as usize] {
          if !seen.contains(connected) {
            stack.push(*connected);
            seen.insert(*connected);
          }
        }
        connections_map[node as usize].clear();
      }
    }
    ans + (n - seen.len() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let links = vec![vec![0, 1], vec![2, 3]];
        let n = 4;
        assert_eq!(count_isolated_communication_groups(&links, n), 2);
    }

    #[test]
    fn test_2() {
        let links = vec![vec![0, 1], vec![1, 2], vec![2, 0]];
        let n = 3;
        assert_eq!(count_isolated_communication_groups(&links, n), 1);
    }

    #[test]
    fn test_3() {
        let links = vec![vec![0, 1], vec![2, 3], vec![3, 4]];
        let n = 5;
        assert_eq!(count_isolated_communication_groups(&links, n), 2);
    }
}

// Example

// Input

// n = 4
// links = [[0, 1], [2, 3]]
// Output

// 2
// Explanation

// There are 4 computers: 0, 1, 2, and 3. Links connect 0-1 and 2-3. Computers 0 and 1 form one group since they are directly connectable. Similarly, 2 and 3 form another group. There are no connections between the two groups, so the number of isolated communication groups is 2.
// Input Format

// The first line contains an integer m denoting the length of links.
// The second line denotes the length of individual elements of array links.
// The next m lines contains individual elements of links.
// The last line contains the value of n.
// Example

// 2
// 2
// 0 1
// 2 3
// 4
