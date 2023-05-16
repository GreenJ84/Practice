// Given an integer array nums and an integer k, return true if there are two distinct indices i and j in the array such that nums[i] == nums[j] and abs(i - j) <= k.

use std::collections::HashMap;
struct Solution {
    
}
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut seen: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            if let Some(_) = seen.get(&nums[i]) {
                return true
            }
            seen.insert(nums[i], 1);
            if i >= k as usize {
                seen.remove(&nums[i - k as usize]);
            }
        }
        false
    }
}
// impl Solution {
//     pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
//         let n = nums.len();
//         let mut window: Vec<i32> = Vec::new();
//         for i in 0..n {
//             println!("{:?} - {}", window, nums[i]);
//             if window.len() == (k + 1) as usize {
//                 window.remove(0);
//             }
//             println!("{:?} - {}", window, nums[i]);
//             if window.contains(&nums[i]){
//                 return true;
//             } else{
//                 window.push(nums[i]);
//             }
//         }
//         false
//     }
// }

fn main() {
    Solution::contains_nearby_duplicate(vec![1,2,3,1], 3);
    Solution::contains_nearby_duplicate(vec![1,0,1,1], 1);
    Solution::contains_nearby_duplicate(vec![1,2,3,1,2,3], 3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_nearby_duplicate() {
        assert_eq!(Solution::contains_nearby_duplicate(vec![1,2,3,1], 3), true);
    }

    #[test]
    fn test_contains_nearby_duplicate2() {
        assert_eq!(Solution::contains_nearby_duplicate(vec![1,0,1,1], 1), true);
    }

    #[test]
    fn test_contains_nearby_duplicate3() {
        assert_eq!(Solution::contains_nearby_duplicate(vec![1,2,3,1,2,3], 2), false);
    }
}