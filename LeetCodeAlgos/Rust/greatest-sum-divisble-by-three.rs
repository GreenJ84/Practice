// Given an integer array nums, return the maximum possible sum of elements of the array such that it is divisible by three.

struct Solution;

impl Solution {
  pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
    let mut r_one = (0i32, Vec::<i32>::new());
    let mut r_two = (0i32, Vec::<i32>::new());
    let mut r_zero = 0;

    for num in &nums {
      match num % 3 {
        1 => {
          r_one.0 += *num;
          r_one.1.push(*num);
        },
        2 => {
          r_two.0 += *num;
          r_two.1.push(*num);
        }
        _ => { r_zero += num },
      }
    }

    let mut ans = 0;
    r_one.1.sort();
    let len_one = r_one.1.len();
    r_two.1.sort();
    let len_two = r_two.1.len();


    for k_one in 0..=2 {
      if k_one > len_one { break; }

      for k_two in 0..=2 {
        if k_two > len_two { break; }
        if ((len_one - k_one).abs_diff(len_two - k_two)) % 3 == 0 { 
          let combined_sum = Self::get_sum(&r_one.1, r_one.0, k_one) + Self::get_sum(&r_two.1, r_two.0, k_two);
          ans = ans.max(combined_sum);
        }
      }
    }

    return ans + r_zero;
  }

  fn get_sum(arr: &Vec<i32>, curr_sum: i32, end: usize) -> i32 {
    if end == 0 { return curr_sum; }
    let mut sum = curr_sum;
    for i in 0..end {
      sum -= arr[i];
    }
    return sum;
  }
}


#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn example_1() {
    let nums = vec![3, 6, 5, 1, 8];
    assert_eq!(Solution::max_sum_div_three(nums), 18);
  }

  #[test]
  fn example_2() {
    let nums = vec![4];
    assert_eq!(Solution::max_sum_div_three(nums), 0);
  }

  #[test]
  fn example_3() {
    let nums = vec![1, 2, 3, 4, 4];
    assert_eq!(Solution::max_sum_div_three(nums), 12);
  }

  #[test]
  fn empty_and_zeros() {
    let empty: Vec<i32> = vec![];
    assert_eq!(Solution::max_sum_div_three(empty), 0);

    let zeros = vec![0, 0, 0];
    assert_eq!(Solution::max_sum_div_three(zeros), 0);
  }

  #[test]
  fn all_divisible_numbers() {
    let nums = vec![3, 6, 9, 12];
    assert_eq!(Solution::max_sum_div_three(nums), 30);
  }

  #[test]
  fn mixed_choose_best_subset() {
    let nums = vec![2, 2, 2, 1];
    // Total sum = 7, best divisible by 3 is 6 (pick three 2s)
    assert_eq!(Solution::max_sum_div_three(nums), 6);
  }
}
