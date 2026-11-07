// You are given a positive integer n representing an n x n cargo deck on a ship. Each cell on the deck can hold one container with a weight of exactly w.

// However, the total weight of all containers, if loaded onto the deck, must not exceed the ship's maximum weight capacity, maxWeight.

// Return the maximum number of containers that can be loaded onto the ship.

// Constraints:
// 1 <= n <= 1000
// 1 <= w <= 1000
// 1 <= maxWeight <= 10^9

struct Solution;
impl Solution {
    pub fn max_containers(n: i32, w: i32, max_weight: i32) -> i32 {
        (n * n).min(max_weight / w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 2;
        let w = 3;
        let max_weight = 15;
        let result = Solution::max_containers(n, w, max_weight);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_2() {
        let n = 3;
        let w = 5;
        let max_weight = 20;
        let result = Solution::max_containers(n, w, max_weight);
        assert_eq!(result, 4);
    }
}
