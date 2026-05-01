// Given n (0-based indexing), return the n-th Fibonacci number where F(0) = 1, F(1) = 2, and F(n) = F(n-1) + F(n-2).

// Constraints
// - 0 <= n <= 92
// - n is an integer

// fn getAutoSaveInterval(n: i32) -> i64 {
fn get_auto_save_interval(n: i32) -> i64 {
  match n {
    0 => 1,
    1 => 2,
    _ => {
      let mut a = 1;
      let mut b = 2;
      for _ in 2..n {
        let temp = a + b;
        a = b;
        b = temp;
      }
      b + a
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 3;
        let expected = 5;
        assert_eq!(get_auto_save_interval(n), expected);
    }

    #[test]
    fn test_2() {
        let n = 10;
        let expected = 144;
        assert_eq!(get_auto_save_interval(n), expected);
    }
}