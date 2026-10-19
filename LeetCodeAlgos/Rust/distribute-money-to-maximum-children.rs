// You are given an integer money denoting the amount of money (in dollars) that you have and another integer children denoting the number of children that you must distribute the money to.

// You have to distribute the money according to the following rules:

// All money must be distributed.
// Everyone must receive at least 1 dollar.
// Nobody receives 4 dollars.
// Return the maximum number of children who may receive exactly 8 dollars if you distribute the money according to the aforementioned rules. If there is no way to distribute the money, return -1.

// Constraints:
// 1 <= money <= 200
// 2 <= children <= 30

struct Solution;
impl Solution {
    pub fn dist_money(money: i32, children: i32) -> i32 {
        if money < children {
            return -1;
        }
        let mut money = money - children;
        let mut res = 0;
        for _ in 0..children {
            if money >= 7 {
                res += 1;
                money -= 7;
            } else {
                break;
            }
        }
        if res == children && money > 0 || res == children - 1 && money == 3 {
            res -= 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let money = 20;
        let children = 3;
        assert_eq!(Solution::dist_money(money, children), 1)
    }

    #[test]
    fn test_2() {
        let money = 16;
        let children = 2;
        assert_eq!(Solution::dist_money(money, children), 2)
    }

    #[test]
    fn test_3() {
        let money = 1;
        let children = 2;
        assert_eq!(Solution::dist_money(money, children), -1)
    }

    #[test]
    fn test_4() {
        let money = 2;
        let children = 2;
        assert_eq!(Solution::dist_money(money, children), 0)
    }

    #[test]
    fn test_5() {
        let money = 23;
        let children = 7;
        assert_eq!(Solution::dist_money(money, children), 2)
    }

        #[test]
    fn test_6() {
        let money = 12;
        let children = 2;
        assert_eq!(Solution::dist_money(money, children), 0)
    }
}
