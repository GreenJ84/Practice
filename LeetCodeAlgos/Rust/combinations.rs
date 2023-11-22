// Given two integers n and k, return all possible combinations of k numbers chosen from the range [1, n].

// You may return the answer in any order.

struct Solution {

}
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();

        fn create_combo(start: i32, n: i32, k: i32, current: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            if current.len() == k as usize {
                ans.push(current.clone());
                return;
            }
            for i in start..=n - (k - current.len() as i32) + 1{
                current.push(i);
                create_combo(i+1, n, k, current, ans);
                current.pop();
            }
        }

        create_combo(1, n, k, &mut Vec::new(), &mut ans);

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine() {
        assert_eq!(Solution::combine(4, 2), vec![vec![1,2],vec![1,3],vec![1,4],vec![2,3],vec![2,4],vec![3,4]]);
    }

    #[test]
    fn test_combine2() {
        assert_eq!(Solution::combine(1, 1), vec![vec![1]]);
    }

    #[test]
    fn test_combine3() {
        assert_eq!(Solution::combine(3, 3), vec![vec![1,2,3]]);
    }

    #[test]
    fn test_combine4() {
        println!("{:?}", Solution::combine(20, 16));
    }
}