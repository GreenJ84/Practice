// You are given a 0-indexed 2D integer array brackets where brackets[i] = [upperi, percenti] means that the ith tax bracket has an upper bound of upperi and is taxed at a rate of percenti. The brackets are sorted by upper bound (i.e. upperi-1 < upperi for 0 < i < brackets.length).

// Tax is calculated as follows:

// The first upper0 dollars earned are taxed at a rate of percent0.
// The next upper1 - upper0 dollars earned are taxed at a rate of percent1.
// The next upper2 - upper1 dollars earned are taxed at a rate of percent2.
// And so on.
// You are given an integer income representing the amount of money you earned. Return the amount of money that you have to pay in taxes. Answers within 10-5 of the actual answer will be accepted.

struct Solution;
impl Solution {
    pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
      println!("{}", income.min(brackets[0][0]));
        let mut taxes = income.min(brackets[0][0]) as f64 * (brackets[0][1] as f64 / 100.0);
        if income <= brackets[0][0] {
            return taxes;
        }
        println!("Initial taxes: {}", taxes);
        for idx in 1..brackets.len() {
            if income <= brackets[idx][0] {
                taxes += (income - brackets[idx - 1][0]) as f64 * (brackets[idx][1] as f64 / 100.0);
                return taxes;
            } else {
                taxes += (brackets[idx][0] - brackets[idx - 1][0]) as f64 * (brackets[idx][1] as f64 / 100.0);
            }
            println!("Taxes after bracket {}: {}", idx, taxes);
        }
        taxes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let brackets = vec![vec![3, 50], vec![7, 10], vec![12, 25]];
        let income = 10;
        let result = Solution::calculate_tax(brackets, income);
        println!("Result: {}", result);
        assert!((result - 2.65).abs() < 1e-5);
    }

    #[test]
    fn test_2() {
        let brackets = vec![vec![1, 0], vec![4, 25], vec![5, 50]];
        let income = 2;
        let result = Solution::calculate_tax(brackets, income);
        println!("Result: {}", result);
        assert!((result - 0.25).abs() < 1e-5);
    }

    #[test]
    fn test_3() {
        let brackets = vec![vec![2, 50]];
        let income = 0;
        let result = Solution::calculate_tax(brackets, income);
        println!("Result: {}", result);
        assert!((result - 0.0).abs() < 1e-5);
    }
}
