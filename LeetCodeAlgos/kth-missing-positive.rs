// Given an array arr of positive integers sorted in a strictly increasing order, and an integer k.
// Return the kth positive integer that is missing from this array.

fn main() {
}
pub struct Solution {}
impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, mut _k: i32) -> i32 {
        let mut curr: usize = 0;
        let mut ans: i32 = 0;
        for i in 1..2001 {
            if curr < arr.len() && arr[curr] == i {
                curr += 1;
                println!("curr: {}, i: {}, _k: {}", curr, i, _k);
            } else {
                _k -= 1;
                println!("curr: {}, i: {}, _k: {}", curr, i, _k);
            }
            if _k == 0{
                ans = i as i32;
                break;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_kth_positive() {
        assert_eq!(Solution::find_kth_positive(vec![2,3,4,7,11], 5), 9);
    }
    #[test]
    fn test_find_kth_positive2() {
        assert_eq!(Solution::find_kth_positive(vec![1, 2, 3, 4], 2), 6);
    }
    #[test]
    fn test_find_kth_positive3() {
        assert_eq!(Solution::find_kth_positive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3), 13);
    }
    #[test]
    fn test_find_kth_positive4() {
        assert_eq!(Solution::find_kth_positive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10), 20);
    }
}
