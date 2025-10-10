// You are given a 0-indexed integer array nums and an integer k.

// You are initially standing at index 0. In one move, you can jump at most k steps forward without going outside the boundaries of the array. That is, you can jump from index i to any index in the range [i + 1, min(n - 1, i + k)] inclusive.

// You want to reach the last index of the array (index n - 1). Your score is the sum of all nums[j] for each index j you visited in the array.

// Return the maximum score you can get.

struct Solution;
impl Solution {
  //! TODO: Optimize with a deque to keep track of max in sliding window
  // pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {

  // }

  // Greedy approach, does not pass all tests
  pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
    let mut curr = nums.len() - 1;
    let mut ans = nums[0] + nums[curr];

    'main: while curr > 0 {
      let mut jump_max = (curr, i32::MIN);
      for run in 1..=k as usize {
        if run > curr { break; }
        let potential = nums[curr - run];
        if potential > 0 {
          curr = curr - run;
          ans += potential;
          continue 'main;
        } else if potential > jump_max.1 {
          jump_max = (curr - run, potential)
        }
      }
      curr = jump_max.0;
      ans += jump_max.1;
    }
    ans
  }
}