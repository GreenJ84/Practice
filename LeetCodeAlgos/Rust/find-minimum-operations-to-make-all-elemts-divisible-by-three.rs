// You are given an integer array nums. In one operation, you can add or subtract 1 from any element of nums.

// Return the minimum number of operations to make all elements of nums divisible by 3.

struct Solution;
impl Solution {
  pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    let mut res = 0i32;
    for num in nums.iter(){
      if num % 3 != 0 { res += 1; }
    }
    res
  }
}