// Given an array of integers nums, find the maximum length of a subarray where the product of all its elements is positive.

// A subarray of an array is a consecutive sequence of zero or more values taken out of that array.

// Return the maximum length of a subarray with positive product.

struct Solution;
impl Solution {
  pub fn get_max_len(nums: Vec<i32>) -> i32 {
    let (mut negatives, mut firstIdx, mut lastIdx) = (0i32, -1isize, -1isize);
    let mut last_zero = -1isize;
    let mut segment_idx = -1isize;
    let mut ans: i32 = 0;

    for idx in 0..nums.len() {
      segment_idx += 1;

      match nums[idx] {
        n if n < 0 => {
          negatives += 1;
          if firstIdx == -1 {
            firstIdx = segment_idx as isize;
          }
          lastIdx = segment_idx as isize;
        },
        // End segment, find longest so far
        0 => {
          ans = ans.max(Self::get_segment_max(
            idx as isize - last_zero - 1,
            negatives,
            firstIdx,
            lastIdx
          ));
          (negatives, firstIdx, lastIdx) = (0, -1, -1);
          last_zero = idx as isize;
          segment_idx = -1isize;
        },
        _ => {},
      }
    }

    // Final segment longest search
    ans = ans.max(Self::get_segment_max(
      segment_idx as isize + 1,
      negatives,
      firstIdx,
      lastIdx
    ));

    ans
  }

  fn get_segment_max(segment_len: isize, negatives_count: i32, firstIdx: isize, lastIdx: isize) -> i32 {
    if segment_len == 0 || negatives_count % 2 == 0 { return segment_len as i32; }

    // If odd negatives, we can either drop the first or last negative
    let drop_first = firstIdx.max(segment_len - firstIdx - 1);
    let drop_last = lastIdx.max(segment_len - lastIdx - 1);
    drop_first.max(drop_last) as i32
  }
}