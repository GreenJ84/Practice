// We are given a list nums of integers representing a list compressed with run-length encoding.

// Consider each adjacent pair of elements [freq, val] = [nums[2*i], nums[2*i+1]] (with i >= 0).  For each such pair, there are freq elements with value val concatenated in a sublist. Concatenate all the sublists from left to right to generate the decompressed list.

// Return the decompressed list.

struct Solution {}
impl Solution {
  pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    let mut i = 0;
    while i < nums.len() {
        let freq = nums[i] as usize;
        let val = nums[i + 1];
        res.extend(std::iter::repeat(val).take(freq));
        i += 2;
    }
    res
  }
}

pub fn main(){
  println!("{:?}", Solution::decompress_rl_elist(vec![1,2,3,4])); // Output: [2,4,4,4]
  println!("{:?}", Solution::decompress_rl_elist(vec![1,1,2,3])); // Output: [1,3,3]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompress_rl_elist() {
      assert_eq(
        Solution::decompress_rl_elist(Vec::from(1,2,3,4)),
        Vec::from(2,4,4,4)
      );
    }
}