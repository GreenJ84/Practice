// At a lemonade stand, each lemonade costs $5. Customers are standing in a queue to buy from you and order one at a time (in the order specified by bills). Each customer will only buy one lemonade and pay with either a $5, $10, or $20 bill. You must provide the correct change to each customer so that the net transaction is that the customer pays $5.

// Note that you do not have any change in hand at first.

// Given an integer array bills where bills[i] is the bill the ith customer pays, return true if you can provide every customer with the correct change, or false otherwise.

struct Solution {}
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let (mut fives, mut tens )= (0, 0);
        for bill in bills.iter() {
            match bill {
                5 => {
                    fives += 1;
                }
                10 => {
                    if fives > 0 {
                        tens += 1;
                        fives -= 1;
                    } else {
                        return false;
                    }
                }
                _ => {
                    if tens > 0 && fives > 0 {
                        tens -= 1;
                        fives -= 1;
                    // If no 10s must have 3 5s
                    } else if fives >= 3 {
                        fives -= 3;
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }
}

// impl Solution {
//     pub fn lemonade_change(bills: Vec<i32>) -> bool {
//         let mut bank = Vec::from([0, 0, 0]);
//         for bill in bills.iter() {
//             match bill {
//                 5 => {
//                     bank[0] += 1;
//                 }
//                 10 => {
//                     bank[1] += 1;
//                     if bank[0] <= 0 {
//                         return false;
//                     }
//                     bank[0] -= 1;
//                 }
//                 _ => {
//                     bank[2] += 1;
//                     // If no 10s must have 3 5s
//                     if bank[1] <= 0 {
//                         // If no 5s cant make change
//                         if bank[0] < 3 {
//                             return false;
//                         } else {
//                             bank[0] -= 3;
//                         }
//                     } else {
//                         bank[1] -= 1;
//                         if bank[0] <= 0 {
//                             return false;
//                         }
//                         bank[0] -= 1;
//                     }
//                 }
//             }
//         }
//         true
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lemonade_change() {
        assert_eq!(true, Solution::lemonade_change(vec![5,5,5,10,20]));
    }

    #[test]
    fn test_lemonade_change2() {
        assert_eq!(false, Solution::lemonade_change(vec![5,5,10,10,20]));
    }
}