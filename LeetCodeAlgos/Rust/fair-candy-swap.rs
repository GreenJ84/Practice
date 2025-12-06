// Alice and Bob have a different total number of candies. You are given two integer arrays aliceSizes and bobSizes where aliceSizes[i] is the number of candies of the ith box of candy that Alice has and bobSizes[j] is the number of candies of the jth box of candy that Bob has.

// Since they are friends, they would like to exchange one candy box each so that after the exchange, they both have the same total amount of candy. The total amount of candy a person has is the sum of the number of candies in each box they have.

// Return an integer array answer where answer[0] is the number of candies in the box that Alice must exchange, and answer[1] is the number of candies in the box that Bob must exchange. If there are multiple answers, you may return any one of them. It is guaranteed that at least one answer exists.

struct Solution;
impl Solution {
  pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
    let sum_alice: i32 = alice_sizes.iter().sum();
    let sum_bob: i32 = bob_sizes.iter().sum();
    let delta = (sum_bob - sum_alice) / 2;

    let bob_sizes_set: std::collections::HashSet<i32> = bob_sizes.into_iter().collect();

    for &alice_candies in &alice_sizes {
      let bob_candies = alice_candies + delta;
      if bob_sizes_set.contains(&bob_candies) {
        return vec![alice_candies, bob_candies];
      }
    }

    vec![]
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example_1() {
    let alice_sizes = vec![1, 1];
    let bob_sizes = vec![2, 2];
    let result = Solution::fair_candy_swap(alice_sizes, bob_sizes);
    assert_eq!(result, vec![1, 2]);
  }

  #[test]
  fn test_example_2() {
    let alice_sizes = vec![1, 2];
    let bob_sizes = vec![2, 3];
    let result = Solution::fair_candy_swap(alice_sizes, bob_sizes);
    assert_eq!(result, vec![1, 2]);
  }

  #[test]
  fn test_example_3() {
    let alice_sizes = vec![2];
    let bob_sizes = vec![1, 3];
    let result = Solution::fair_candy_swap(alice_sizes, bob_sizes);
    assert_eq!(result, vec![2, 3]);
  }
}
