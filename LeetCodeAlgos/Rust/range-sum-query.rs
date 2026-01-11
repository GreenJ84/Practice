// Given an integer array nums, handle multiple queries of the following types:

// Update the value of an element in nums.
// Calculate the sum of the elements of nums between indices left and right inclusive where left <= right.
// Implement the NumArray class:

// NumArray(int[] nums) Initializes the object with the integer array nums.
// void update(int index, int val) Updates the value of nums[index] to be val.
// int sumRange(int left, int right) Returns the sum of the elements of nums between indices left and right inclusive (i.e. nums[left] + nums[left + 1] + ... + nums[right]).

struct NumArray {
  nums: Vec<i32>,
  pre_sum: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut pre_sum = nums.clone();
        for i in 1..pre_sum.len() {
            pre_sum[i] += pre_sum[i - 1];
        }
        NumArray { nums, pre_sum }
    }

    fn update(&mut self, index: i32, val: i32) {
      let diff = val - self.nums[index as usize];
      self.nums[index as usize] = val; // 0 - n

      for idx in index as usize..self.pre_sum.len() {
        self.pre_sum[idx] += diff;
      }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left == 0 {
          return self.pre_sum[right as usize];
        }
        self.pre_sum[right as usize] - self.pre_sum.get(left as usize - 1).unwrap_or(&0)
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example_1() {
    let mut obj = NumArray::new(vec![1, 3, 5]);
    assert_eq!(obj.sum_range(0, 2), 9);
    obj.update(1, 2);
    assert_eq!(obj.sum_range(0, 2), 8);
  }

  #[test]
  fn test_single_element() {
    let mut obj = NumArray::new(vec![5]);
    assert_eq!(obj.sum_range(0, 0), 5);
    obj.update(0, 10);
    assert_eq!(obj.sum_range(0, 0), 10);
  }

  #[test]
  fn test_negative_numbers() {
    let mut obj = NumArray::new(vec![-100, -50, 0, 50, 100]);
    assert_eq!(obj.sum_range(0, 4), 0);
    obj.update(0, 100);
    assert_eq!(obj.sum_range(0, 4), 200);
  }

  #[test]
  fn test_multiple_updates() {
    let mut obj = NumArray::new(vec![1, 2, 3, 4, 5]);
    assert_eq!(obj.sum_range(1, 3), 9);
    obj.update(2, 10);
    assert_eq!(obj.sum_range(1, 3), 16);
    obj.update(1, 20);
    assert_eq!(obj.sum_range(0, 4), 40);
  }

  #[test]
  fn test_boundary_indices() {
    let mut obj = NumArray::new(vec![10, 20, 30, 40, 50]);
    assert_eq!(obj.sum_range(0, 0), 10);
    assert_eq!(obj.sum_range(4, 4), 50);
    obj.update(0, -100);
    assert_eq!(obj.sum_range(0, 4), 40);
  }
}
