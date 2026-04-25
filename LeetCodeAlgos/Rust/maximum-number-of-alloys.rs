// You are the owner of a company that creates alloys using various types of metals. There are n different types of metals available, and you have access to k machines that can be used to create alloys. Each machine requires a specific amount of each metal type to create an alloy.

// For the ith machine to create an alloy, it needs composition[i][j] units of metal of type j. Initially, you have stock[i] units of metal type i, and purchasing one unit of metal type i costs cost[i] coins.

// Given integers n, k, budget, a 1-indexed 2D array composition, and 1-indexed arrays stock and cost, your goal is to maximize the number of alloys the company can create while staying within the budget of budget coins.

// All alloys must be created with the same machine.

// Return the maximum number of alloys that the company can create.

struct Solution;
impl Solution {
      pub fn max_number_of_alloys(
      n: i32,
      k: i32,
      budget: i32,
      composition: Vec<Vec<i32>>,
      stock: Vec<i32>,
      cost: Vec<i32>,
    ) -> i32 {
        let budget = budget as i64;

        // Can a machine make a certain number of alloys with the current stock and budget?
        let can_make = |machine: usize, alloys: i64| -> bool {
            let mut required = 0i64;

            for material in 0..n as usize {
                let needed = alloys * composition[machine][material] as i64;
                let extra = needed - stock[material] as i64;

                if extra > 0 {
                    required += extra * cost[material] as i64;
                    if required > budget {
                        return false;
                    }
                }
            }

            true
        };

        let mut maximum_alloys = 0i64;
        // Exponential search to find an upper bound for the number of alloys each machine can produce, then binary search to find the exact maximum.
        for machine in 0..k as usize {
            let mut left = 0i64;
            let mut right = 1i64;

            // Exponential search to find an upper bound for the number of alloys each machine can produce
            while can_make(machine, right) {
                left = right;
                right *= 2;
            }

            // Binary search last range to find the exact maximum.
            while left < right {
                let mid = left + (right - left + 1) / 2;
                if can_make(machine, mid) {
                    left = mid;
                } else {
                    right = mid - 1;
                }
            }
            maximum_alloys = maximum_alloys.max(left);
        }

        maximum_alloys as i32
    }

    // This is a greedy approach fails to account for stock to budget to composition optimization and the edge cases it brings.
    // Efficiently finding each machine's maximum alloys producible with the stock to budget efficiently is the key to solving this problem.
    pub fn _max_number_of_alloys1(
        n: i32,
        k: i32,
        mut budget: i32,
        composition: Vec<Vec<i32>>,
        mut stock: Vec<i32>,
        cost: Vec<i32>,
    ) -> i32 {
        // Find lowest per-alloy-cost and the machine that has it.
        let (lowest_cost, machine) = (0..k as usize).fold((i32::MAX, 0usize), |(low, idx), mach| {
            let machine_cost = (0..n as usize)
                .map(|material| composition[mach][material] * cost[material])
                .sum();

            if machine_cost < low {
                (machine_cost, mach)
            } else {
                (low, idx)
            }
        });
        println!("lowest_cost: {lowest_cost}, machine: {machine}");

        // Find the maximum number of alloys that can be created with the current stock.
        let mut ans = (0..n as usize)
            .map(|material| stock[material] / composition[machine][material])
            .min()
            .unwrap();
        println!("stock: {:?} / composition: {:?}", stock, composition[machine]);
        println!("max alloys with current stock: {ans}");

        // Subtract the materials used to create those alloys from the stock.
        for material in 0..n as usize {
            stock[material] -= ans * composition[machine][material];
        }

        // If no materials in stock, lowest_cost is the full cost of one alloy
        let mut oos = false;
        // Keep creating alloys until we go negative, then refund the last one
        while budget > 0 {
            if !oos {
                let alloy_cost = (0..n as usize)
                    .map(|material| {
                        if stock[material] >= composition[machine][material] {
                            0
                        } else {
                          let cost = (composition[machine][material] - stock[material]) * cost[material];
                          stock[material] = 0;
                          cost
                        }
                    })
                    .sum::<i32>();
                budget -= alloy_cost;
                if alloy_cost == lowest_cost {
                    oos = true;
                }
            } else {
                budget -= lowest_cost;
            }
            ans += 1;
        }
        if budget == 0 {
          ans
        } else {
          ans - 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 3;
        let k = 2;
        let budget = 15;
        let composition = vec![vec![1, 1, 1], vec![1, 1, 10]];
        let stock = vec![0, 0, 0];
        let cost = vec![1, 2, 3];
        assert_eq!(
            Solution::max_number_of_alloys(n, k, budget, composition, stock, cost),
            2
        );
    }

    #[test]
    fn test2() {
        let n = 3;
        let k = 2;
        let budget = 15;
        let composition = vec![vec![1, 1, 1], vec![1, 1, 10]];
        let stock = vec![0, 0, 100];
        let cost = vec![1, 2, 3];
        assert_eq!(
            Solution::max_number_of_alloys(n, k, budget, composition, stock, cost),
            5
        );
    }

    #[test]
    fn test3() {
        let n = 2;
        let k = 3;
        let budget = 10;
        let composition = vec![vec![2, 1], vec![1, 2], vec![1, 1]];
        let stock = vec![1, 1];
        let cost = vec![5, 5];
        assert_eq!(
            Solution::max_number_of_alloys(n, k, budget, composition, stock, cost),
            2
        );
    }
}
