// There are numBottles water bottles that are initially full of water. You can exchange numExchange empty water bottles from the market with one full water bottle.

// The operation of drinking a full water bottle turns it into an empty bottle.

// Given the two integers numBottles and numExchange, return the maximum number of water bottles you can drink.

struct Solution {}
impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut ans: i32 = 0;
        let mut empties: i32 = 0;
        while num_bottles > 0 {
            ans += num_bottles;
            empties += num_bottles;
            num_bottles = empties / num_exchange;
            empties %= num_exchange;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_water_bottles() {
        assert_eq!(Solution::num_water_bottles(9, 3), 13);
    }

    #[test]
    fn test_num_water_bottles_2() {
        assert_eq!(Solution::num_water_bottles(15, 4), 19);
    }
}