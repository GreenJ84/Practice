// You are given a rectangular cake of size h x w and two arrays of integers horizontalCuts and verticalCuts where:

// horizontalCuts[i] is the distance from the top of the rectangular cake to the ith horizontal cut and similarly, and
// verticalCuts[j] is the distance from the left of the rectangular cake to the jth vertical cut.
// Return the maximum area of a piece of cake after you cut at each horizontal and vertical position provided in the arrays horizontalCuts and verticalCuts. Since the answer can be a large number, return this modulo 109 + 7.

// Constraints:
// - 2 <= h, w <= 109
// - 1 <= horizontalCuts.length <= min(h - 1, 105)
// - 1 <= verticalCuts.length <= min(w - 1, 105)
// - 1 <= horizontalCuts[i] < h
// - 1 <= verticalCuts[i] < w
// - All the elements in horizontalCuts are distinct.
// - All the elements in verticalCuts are distinct.

struct Solution;
impl Solution {
    pub fn max_area(
        h: i32,
        w: i32,
        mut horizontal_cuts: Vec<i32>,
        mut vertical_cuts: Vec<i32>,
    ) -> i32 {
      horizontal_cuts.sort_unstable();
      vertical_cuts.sort_unstable();
      let mut last = 0i64;

        let mut max_rows = 0i64;
        for &ho in horizontal_cuts.iter() {
          max_rows = max_rows.max(ho as i64 - last);
          last = ho as i64;
        }
        max_rows = max_rows.max(h as i64 - last);
        last = 0i64;

        let mut max_cols = 0i64;
        for &ve in vertical_cuts.iter() {
          max_cols = max_cols.max(ve as i64 - last);
          last = ve as i64;
        }
        max_cols = max_cols.max(w as i64 - last);
        println!("max_rows: {}, max_cols: {}", max_rows, max_cols);

        (max_rows * max_cols % 1_000_000_007) as i32
    }

    pub fn _max_area1(
        h: i32,
        w: i32,
        mut horizontal_cuts: Vec<i32>,
        mut vertical_cuts: Vec<i32>,
    ) -> i32 {
        horizontal_cuts.sort_unstable();
        vertical_cuts.sort_unstable();
        let mut ans = 0i64;

        let mut top = 0i64;
        let mut left = 0i64;
        for &ho in horizontal_cuts.iter() {
            let height = ho as i64 - top;
            for &ve in vertical_cuts.iter() {
                ans = ans.max(height * (ve as i64 - left));
                left = ve as i64;
            }
            ans = ans.max(height * (w as i64 - left));
            top = ho as i64;
            left = 0i64;
        }
        for &ve in vertical_cuts.iter() {
            ans = ans.max((h as i64 - top) * (ve as i64 - left));
            left = ve as i64;
        }
        ans = ans.max((h as i64 - top) * (w as i64 - left));
        (ans % 1_000_000_007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let h = 5;
        let w = 4;
        let horizontal_cuts = vec![1, 2, 4];
        let vertical_cuts = vec![1, 3];
        let result = Solution::max_area(h, w, horizontal_cuts, vertical_cuts);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_2() {
        let h = 5;
        let w = 4;
        let horizontal_cuts = vec![3, 1];
        let vertical_cuts = vec![1];
        let result = Solution::max_area(h, w, horizontal_cuts, vertical_cuts);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_3() {
        let h = 5;
        let w = 4;
        let horizontal_cuts = vec![3];
        let vertical_cuts = vec![3];
        let result = Solution::max_area(h, w, horizontal_cuts, vertical_cuts);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_4() {
        let h = 1_000_000_000;
        let w = 1_000_000_000;
        let horizontal_cuts = vec![2];
        let vertical_cuts = vec![2];
        let result = Solution::max_area(h, w, horizontal_cuts, vertical_cuts);
        assert_eq!(result, 81);
    }

    #[test]
    fn test_5() {
        let h = 7;
        let w = 3;
        let horizontal_cuts = vec![3,4,2];
        let vertical_cuts = vec![2];
        let result = Solution::max_area(h, w, horizontal_cuts, vertical_cuts);
        assert_eq!(result, 6);
    }
}
