// There are n mountains in a row, and each mountain has a height. You are given an integer array height where height[i] represents the height of mountain i, and an integer threshold.

// A mountain is called stable if the mountain just before it (if it exists) has a height strictly greater than threshold. Note that mountain 0 is not stable.

// Return an array containing the indices of all stable mountains in any order.

struct Solution;
impl Solution {
    pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        for idx in 1..height.len() {
          if height[idx - 1] > threshold {
            ans.push(idx as i32);
          }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let height = vec![1,2,3,4,5];
        let threshold = 2;
        assert_eq!(Solution::stable_mountains(height, threshold), vec![3,4]);
    }

    #[test]
    fn test_2() {
        let height = vec![10,1,10,1,10];
        let threshold = 3;
        assert_eq!(Solution::stable_mountains(height, threshold), vec![1,3]);
    }

    #[test]
    fn test_3() {
        let height = vec![10,1,10,1,10];
        let threshold = 10;
        assert_eq!(Solution::stable_mountains(height, threshold), vec![]);
    }
}