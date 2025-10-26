// A permutation of an array of integers is an arrangement of its members into a sequence or linear order.

// For example, for arr = [1,2,3], the following are all the permutations of arr: [1,2,3], [1,3,2], [2, 1, 3], [2, 3, 1], [3,1,2], [3,2,1].
// The next permutation of an array of integers is the next lexicographically greater permutation of its integer. More formally, if all the permutations of the array are sorted in one container according to their lexicographical order, then the next permutation of that array is the permutation that follows it in the sorted container. If such arrangement is not possible, the array must be rearranged as the lowest possible order (i.e., sorted in ascending order).

// For example, the next permutation of arr = [1,2,3] is [1,3,2].
// Similarly, the next permutation of arr = [2,3,1] is [3,1,2].
// While the next permutation of arr = [3,2,1] is [1,2,3] because [3,2,1] does not have a lexicographical larger rearrangement.
// Given an array of integers nums, find the next permutation of nums.

// The replacement must be in place and use only constant extra memory.

struct Solution;

use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
      let  n = nums.len();
      if n < 2 { return; }

      let mut largest = -1;
      let mut seen = BinaryHeap::<Reverse<(i32, usize)>>::new();
      // See if we can find the dip
      for idx in (0..n).rev() {
        if nums[idx] < largest {
            while !seen.is_empty() {
                let (val, i) = seen.pop().unwrap().0;
                if val > nums[idx] {
                    let temp = nums[idx];
                    nums[idx] = val;
                    nums[i] = temp;
                    let mut tail = nums[idx+1..].to_vec();
                    tail.sort();
                    for (j, &v) in tail.iter().enumerate() {
                        nums[idx + 1 + j] = v;
                    }
                    return;
                }
            }
        } else if nums[idx] > largest {
            largest = nums[idx];
            seen.push(Reverse((nums[idx], idx)))
        }

      }
      // If all ascending front to back, reverse
      nums.reverse()
    }
}