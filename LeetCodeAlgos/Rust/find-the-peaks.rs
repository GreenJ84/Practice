// You are given a 0-indexed array mountain. Your task is to find all the peaks in the mountain array.

// Return an array that consists of indices of peaks in the given array in any order.

// Constraints:
// - 3 <= mountain.length <= 100
// - 1 <= mountain[i] <= 100

struct Solution;
impl Solution {
    pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
      let mut ans = Vec::new();
      for i in 1..mountain.len() - 1 {
        if mountain[i - 1] < mountain[i] && mountain[i] > mountain[i + 1]{
          ans.push(i as i32);
        }
      }
      ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mountain = vec![2, 4, 4];
        let expected = vec![];
        assert_eq!(Solution::find_peaks(mountain), expected);
    }

    #[test]
    fn test2() {
        let mountain = vec![1, 4, 3, 8, 5];
        let expected = vec![1, 3];
        assert_eq!(Solution::find_peaks(mountain), expected);
    }
}
