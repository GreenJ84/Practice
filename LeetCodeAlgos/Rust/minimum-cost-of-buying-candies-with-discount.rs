// A shop is selling candies at a discount. For every two candies sold, the shop gives a third candy for free.
//
// The customer can choose any candy to take away for free as long as the cost of the chosen candy is less than or equal to the minimum cost of the two candies bought.
//
// For example, if there are 4 candies with costs 1, 2, 3, and 4, and the customer buys candies with costs 2 and 3, they can take the candy with cost 1 for free, but not the candy with cost 4.
// Given a 0-indexed integer array cost, where cost[i] denotes the cost of the ith candy, return the minimum cost of buying all the candies.
//

struct Solution {}
impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        let mut min = 0;
        cost.sort();
        for (idx, val) in cost.into_iter().rev().enumerate(){
            if (idx + 1) % 3 != 0 {
                min += val;
            }
        }
        min
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    pub fn test1(){
        assert_eq!(
            Solution::minimum_cost(Vec::from([1,2,3])),
            5
        );
    }

    #[test]
    pub fn test2(){
        assert_eq!(
            Solution::minimum_cost(Vec::from([6,5,7,9,2,2])),
            23
        );
    }

    #[test]
    pub fn test3(){
        assert_eq!(
            Solution::minimum_cost(Vec::from([5,5])),
            10
        );
    }
}