// You are given an array apple of size n and an array capacity of size m.

// There are n packs where the ith pack contains apple[i] apples. There are m boxes as well, and the ith box has a capacity of capacity[i] apples.

// Return the minimum number of boxes you need to select to redistribute these n packs of apples into boxes.

// Note that, apples from the same pack can be distributed into different boxes.

// Constraints:
// 1 <= n == apple.length <= 50
// 1 <= m == capacity.length <= 50
// 1 <= apple[i], capacity[i] <= 50
// The input is generated such that it's possible to redistribute packs of apples into boxes.

struct Solution;
impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        capacity.sort_unstable_by(|a, b| b.cmp(a));
        let mut apples: i32 = apple.into_iter().sum();
        let mut bx = 0usize;
        while apples > 0 {
            apples -= capacity[bx];
            bx += 1;
        }
        bx as i32
    }

    pub fn minimum_boxes1(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        capacity.sort_unstable_by(|a, b| b.cmp(a));
        let mut apples: i32 = apple.into_iter().sum();
        for idx in 0..capacity.len() {
            apples -= capacity[idx];
            if apples <= 0 {
                return idx as i32 + 1;
            }
        }
        capacity.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let apples = vec![1, 3, 2];
        let cap = vec![4, 3, 1, 5, 2];
        assert_eq!(Solution::minimum_boxes(apples, cap), 2);
    }

    #[test]
    fn test_2() {
        let apples = vec![5, 5, 5];
        let cap = vec![2, 4, 2, 7];
        assert_eq!(Solution::minimum_boxes(apples, cap), 4);
    }
}
