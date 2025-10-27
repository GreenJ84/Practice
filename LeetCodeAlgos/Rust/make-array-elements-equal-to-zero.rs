// You are given an integer array nums.

// Start by selecting a starting position curr such that nums[curr] == 0, and choose a movement direction of either left or right.

// After that, you repeat the following process:

// If curr is out of the range [0, n - 1], this process ends.
// If nums[curr] == 0, move in the current direction by incrementing curr if you are moving right, or decrementing curr if you are moving left.
// Else if nums[curr] > 0:
// Decrement nums[curr] by 1.
// Reverse your movement direction (left becomes right and vice versa).
// Take a step in your new direction.
// A selection of the initial position curr and movement direction is considered valid if every element in nums becomes 0 by the end of the process.

// Return the number of possible valid selections.

struct Solution;
impl Solution {
  pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut running_sums = Vec::<i32>::new();

    for num in nums {
      if num == 0 {
        running_sums.push(sum);
      } else {
        sum += num;
      }
    }

    let mut ans = 0;
    for front_sum in running_sums {
      let rest = sum - front_sum;
      match (rest - front_sum).abs() {
        0 => {
          ans += 2;
        },
        1 => {
          ans += 1;
        },
        _ => {}
      }
    }
    ans
  }
}

