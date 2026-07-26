// You are given two 2D integer arrays nums1 and nums2.

// nums1[i] = [idi, vali] indicate that the number with the id idi has a value equal to vali.
// nums2[i] = [idi, vali] indicate that the number with the id idi has a value equal to vali.
// Each array contains unique ids and is sorted in ascending order by id.

// Merge the two arrays into one array that is sorted in ascending order by id, respecting the following conditions:

// Only ids that appear in at least one of the two arrays should be included in the resulting array.
// Each id should be included only once and its value should be the sum of the values of this id in the two arrays. If the id does not exist in one of the two arrays, then assume its value in that array to be 0.
// Return the resulting array. The returned array must be sorted in ascending order by id.

// Constraints:
// 1 <= nums1.length, nums2.length <= 200
// nums1[i].length == nums2[j].length == 2
// 1 <= idi, vali <= 1000
// Both arrays contain unique ids.
// Both arrays are in strictly ascending order by id.

struct Solution;
impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
      let (mut idx1, n1) = (0usize, nums1.len());
      let (mut idx2, n2) = (0usize, nums2.len());

      let mut ans: Vec<Vec<i32>> = vec![];
      while idx1 < n1 && idx2 < n2 {
        match (nums1[idx1][0], nums2[idx2][0]){
          (x, y) if x < y => {
            ans.push(nums1[idx1].clone());
            idx1 += 1;
          },
          (x, y) if x == y => {
            ans.push(vec![nums1[idx1][0], nums1[idx1][1] + nums2[idx2][1]]);
            idx1 += 1;
            idx2 += 1;
          },
          (_, _) => {
            ans.push(nums2[idx2].clone());
            idx2 += 1;
          }
        }
      }

      let (bool1, bool2) = (idx1 == n1, idx2 == n2);
      if !bool1 && bool2 {
        while idx1 < n1 {
          ans.push(nums1[idx1].clone());
          idx1 += 1;
        }
      } else if bool1 && !bool2 {
        while idx2 < n2 {
          ans.push(nums2[idx2].clone());
          idx2 += 1;
        }
      }
      ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums1 = vec![vec![1, 2], vec![2, 3], vec![4, 5]];
        let nums2 = vec![vec![1, 4], vec![3, 2], vec![4, 1]];
        let expected = vec![vec![1, 6], vec![2, 3], vec![3, 2], vec![4, 6]];
        assert_eq!(Solution::merge_arrays(nums1, nums2), expected);
    }

    #[test]
    fn test_2() {
        let nums1 = vec![vec![2, 4], vec![3, 6], vec![5, 5]];
        let nums2 = vec![vec![1, 3], vec![4, 3]];
        let expected = vec![vec![1, 3], vec![2, 4], vec![3, 6], vec![4, 3], vec![5, 5]];
        assert_eq!(Solution::merge_arrays(nums1, nums2), expected);
    }
}
