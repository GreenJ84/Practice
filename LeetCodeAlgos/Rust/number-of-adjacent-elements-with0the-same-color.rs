// You are given an integer n representing an array colors of length n where all elements are set to 0's meaning uncolored. You are also given a 2D integer array queries where queries[i] = [indexi, colori]. For the ith query:

// Set colors[indexi] to colori.
// Count the number of adjacent pairs in colors which have the same color (regardless of colori).
// Return an array answer of the same length as queries where answer[i] is the answer to the ith query.

// Constraints:
// 1 <= n <= 105
// 1 <= queries.length <= 105
// queries[i].length == 2
// 0 <= indexi <= n - 1
// 1 <=  colori <= 105

struct Solution;
impl Solution {
    pub fn color_the_array(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut colors = vec![0; n];
        let mut pairs = 0;
        queries
            .iter()
            .map(|q| {
                let (i, c) = (q[0] as usize, q[1]);
                let oc = colors[i];
                colors[i] = c;
                pairs += Self::check(&colors, n, i, c, oc);
                pairs
            })
            .collect::<Vec<i32>>()
    }

    fn check(colors: &Vec<i32>, n: usize, i: usize, c: i32, oc: i32) -> i32 {
        let mut adj = 0;
        if i > 0 && colors[i - 1] != 0 {
            adj += (colors[i - 1] == c) as i32;
            adj -= (colors[i - 1] == oc) as i32;
        }
        if i < n - 1 && colors[i + 1] != 0 {
            adj += (colors[i + 1] == c) as i32;
            adj -= (colors[i + 1] == oc) as i32;
        }
        adj
    }

    pub fn color_the_array1(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut colors = vec![0; n];
        let mut pairs = 0;
        queries
            .iter()
            .map(|q| {
                let (i, c) = (q[0] as usize, q[1]);
                pairs -= Self::check1(&colors, n, i, colors[i]);
                colors[i] = c;
                pairs += Self::check1(&colors, n, i, c);
                pairs
            })
            .collect::<Vec<i32>>()
    }

    fn check1(colors: &Vec<i32>, n: usize, i: usize, c: i32) -> i32 {
        if c == 0 {
            return 0;
        }
        let mut adj = 0;
        if i > 0 && colors[i - 1] == c {
            adj += 1;
        }
        if i < n - 1 && colors[i + 1] == c {
            adj += 1
        }
        adj
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 4;
        let queries = vec![vec![0, 2], vec![1, 2], vec![3, 1], vec![1, 1], vec![2, 1]];
        assert_eq!(Solution::color_the_array(n, queries), vec![0, 1, 1, 0, 2]);
    }

    #[test]
    fn test_2() {
        let n = 1;
        let queries = vec![vec![0, 100000]];
        assert_eq!(Solution::color_the_array(n, queries), vec![0]);
    }
}
