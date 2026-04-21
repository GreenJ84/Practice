// Given a sorted array of positive integers and a target value, count the number of pairs (i, j) where i < j and array[i] + array[j] <= target.

// fn countAffordablePairs(prices: &[i32], budget: i32) -> i32 {
fn count_affordable_pairs(prices: &[i32], budget: i32) -> i32 {
    if prices.len() < 2 {
        return 0;
    }

    let mut count = 0;
    let mut left = 0;
    let mut right = prices.len() - 1;

    while left < right {
        if prices[left] + prices[right] <= budget {
            count += (right - left) as i32;
            left += 1; // Move left pointer to the right
        } else {
            right -= 1; // Move right pointer to the left
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_affordable_pairs() {
        let prices = [1, 2, 3, 4, 5];
        let budget = 7;
        assert_eq!(count_affordable_pairs(&prices, budget), 8);
    }

    #[test]
    fn test_count_affordable_pairs_empty() {
        let prices: [i32; 0] = [];
        let budget = 5;
        assert_eq!(count_affordable_pairs(&prices, budget), 0);
    }

    #[test]
    fn test_count_affordable_pairs_no_pairs() {
        let prices = [10, 20, 30];
        let budget = 5;
        assert_eq!(count_affordable_pairs(&prices, budget), 0);
    }
}

// Example

// Input:

// prices = [1, 2, 3, 4, 5]
// budget = 7
// Output:

// 8
// Explanation:

// We need pairs (i, j) with i < j and prices[i] + prices[j] ≤ 7. List all pairs:

// (1, 2) = 3 ≤ 7
// (1, 3) = 4 ≤ 7
// (1, 4) = 5 ≤ 7
// (1, 5) = 6 ≤ 7
// (2, 3) = 5 ≤ 7
// (2, 4) = 6 ≤ 7
// (2, 5) = 7 ≤ 7
// (3, 4) = 7 ≤ 7
// Pairs like (3,5)=8, (4,5)=9 exceed the budget. Total valid pairs = 8.
