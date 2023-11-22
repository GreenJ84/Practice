// You are given an integer array nums. We consider an array good if it is a permutation of an array base[n].

// base[n] = [1, 2, ..., n - 1, n, n] (in other words, it is an array of length n + 1 which contains 1 to n - 1 exactly once, plus two occurrences of n). For example, base[1] = [1, 1] and base[3] = [1, 2, 3, 3].

// Return true if the given array is good, otherwise return false.

// Note: A permutation of integers represents an arrangement of these numbers.

struct Solution {}
impl Solution {
    pub fn is_good(mut nums: Vec<i32>) -> bool {
        nums.sort_unstable();
        let k = (nums.len() - 1) as usize;
        for i in 0..k {
            if *nums.get(i).unwrap() != (i + 1) as i32 {
                return false;
            }
        }
        if *nums.get(k).unwrap() != k as i32 {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_good() {
        assert_eq!(Solution::is_good(vec![2, 1, 3]), false);
    }

    #[test]
    fn test_is_good2() {
        assert_eq!(Solution::is_good(vec![1,3,3,2]), true);
    }

    #[test]
    fn test_is_good3() {
        assert_eq!(Solution::is_good(vec![1,1]), true);
    }

    #[test]
    fn test_is_good4() {
        assert_eq!(Solution::is_good(vec![3, 4, 4, 1, 2, 1]), false);
    }
}