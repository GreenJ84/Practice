// You are given a non-negative floating point number rounded to two decimal places celsius, that denotes the temperature in Celsius.

// You should convert Celsius into Kelvin and Fahrenheit and return it as an array ans = [kelvin, fahrenheit].

// Return the array ans. Answers within 10-5 of the actual answer will be accepted.

// Note that:
// Kelvin = Celsius + 273.15
// Fahrenheit = Celsius * 1.80 + 32.00

// Constraints:
// 0 <= celsius <= 1000

struct Solution;
impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        vec![celsius + 273.15, celsius * 1.8 + 32.0]
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let celsius = 36.50;
    let expected = vec![309.65000, 97.70000];
    let result = Solution::convert_temperature(celsius);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_2() {
    let celsius = 122.11;
    let expected = vec![395.26000, 251.79800];
    let result = Solution::convert_temperature(celsius);
    assert_eq!(result, expected);
  }
}
