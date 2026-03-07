// You are given two 2D integer arrays, items1 and items2, representing two sets of items. Each array items has the following properties:

// items[i] = [valuei, weighti] where valuei represents the value and weighti represents the weight of the ith item.
// The value of each item in items is unique.
// Return a 2D integer array ret where ret[i] = [valuei, weighti], with weighti being the sum of weights of all items with value valuei.

// Note: ret should be returned in ascending order by value.

struct Solution;
impl Solution {
    pub fn merge_similar_items(
        mut items1: Vec<Vec<i32>>,
        mut items2: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        items1.sort_unstable_by_key(|a| a[0]);
        items2.sort_unstable_by_key(|a| a[0]);

        return match items1.len().cmp(&items2.len()) {
            std::cmp::Ordering::Less => Self::merge_sorted(items2, items1),
            _ => Self::merge_sorted(items1, items2),
        };
    }

    fn merge_sorted(main: Vec<Vec<i32>>, merge: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut new = Vec::<Vec<i32>>::new();
        let mut last = vec![0i32, 0i32];
        let (mut idx, len) = (0usize, merge.len());

        for item in main {
            while idx < len && item[0] >= merge[idx][0] {
                if merge[idx][0] == last[0] {
                    last[1] += merge[idx][1];
                } else {
                    if last[0] > 0 {
                        new.push(last);
                    }
                    last = merge[idx].clone();
                }
                idx += 1;
            }

            if item[0] == last[0] {
                last[1] += item[1];
            } else {
                if last[0] > 0 {
                    new.push(last);
                }
                last = item;
            }
        }
        for idx in idx..len {
            if merge[idx][0] == last[0] {
                last[1] += merge[idx][1];
            } else {
                if last[0] > 0 {
                    new.push(last);
                }
                last = merge[idx].clone();
            }
        }
        new.push(last);
        new
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let items1 = vec![vec![1, 1], vec![4, 5], vec![3, 8]];
        let items2 = vec![vec![3, 1], vec![1, 5]];
        let expected = vec![vec![1, 6], vec![3, 9], vec![4, 5]];
        assert_eq!(Solution::merge_similar_items(items1, items2), expected);
    }

    #[test]
    fn test_2() {
        let items1 = vec![vec![1, 1], vec![3, 2], vec![2, 3]];
        let items2 = vec![vec![2, 1], vec![3, 2], vec![1, 3]];
        let expected = vec![vec![1, 4], vec![2, 4], vec![3, 4]];
        assert_eq!(Solution::merge_similar_items(items1, items2), expected);
    }

    #[test]
    fn test_3() {
        let items1 = vec![vec![1, 3], vec![2, 2]];
        let items2 = vec![vec![7, 1], vec![2, 2], vec![1, 4]];
        let expected = vec![vec![1, 7], vec![2, 4], vec![7, 1]];
        assert_eq!(Solution::merge_similar_items(items1, items2), expected);
    }
}
