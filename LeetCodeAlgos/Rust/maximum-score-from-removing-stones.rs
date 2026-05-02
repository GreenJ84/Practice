// You are playing a solitaire game with three piles of stones of sizes a​​​​​​, b,​​​​​​ and c​​​​​​ respectively. Each turn you choose two different non-empty piles, take one stone from each, and add 1 point to your score. The game stops when there are fewer than two non-empty piles (meaning there are no more available moves).

// Given three integers a​​​​​, b,​​​​​ and c​​​​​, return the maximum score you can get.

// Constraints:
// - 1 <= a, b, c <= 105

struct Solution;
impl Solution {
    pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
        let mut scores = vec![a, b, c];
        scores.sort_unstable();

        let mut ans = 0;
        while scores[0] > 0 && scores[2] > 0 {
          ans += 1;
          scores[2] -= 1;
          scores[1] -= 1;

          if scores[2] > 0 {
            ans += 1;
            scores[0] -= 1;
            if scores[2] >= scores[1] {
              scores[2] -= 1;
            } else {
              scores[1] -= 1;
            }
          }
          if scores[0] * 2 > scores[1] + scores[2] {
            scores.sort_unstable();
          }
        }

        match (scores[0], scores[2]) {
          (0, f) | (f, 0) => {
            ans + f.min(scores[1])
          },
          _ => { ans }
        }
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let a = 2;
    let b = 4;
    let c = 6;
    assert_eq!(Solution::maximum_score(a, b, c), 6)
  }

  #[test]
  fn test_2() {
    let a = 4;
    let b = 4;
    let c = 6;
    assert_eq!(Solution::maximum_score(a, b, c), 7)
  }

  #[test]
  fn test_3() {
    let a = 1;
    let b = 8;
    let c = 8;
    assert_eq!(Solution::maximum_score(a, b, c), 8)
  }

  #[test]
  fn test_4() {
    let a = 8;
    let b = 16;
    let c = 22;
    assert_eq!(Solution::maximum_score(a, b, c), 23)
  }

  #[test]
  fn test_5() {
    let a = 19;
    let b = 24;
    let c = 24;
    assert_eq!(Solution::maximum_score(a, b, c), 33)
  }
}
