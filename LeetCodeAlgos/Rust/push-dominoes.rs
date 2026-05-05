// There are n dominoes in a line, and we place each domino vertically upright. In the beginning, we simultaneously push some of the dominoes either to the left or to the right.

// After each second, each domino that is falling to the left pushes the adjacent domino on the left. Similarly, the dominoes falling to the right push their adjacent dominoes standing on the right.

// When a vertical domino has dominoes falling on it from both sides, it stays still due to the balance of the forces.

// For the purposes of this question, we will consider that a falling domino expends no additional force to a falling or already fallen domino.

// You are given a string dominoes representing the initial state where:

// dominoes[i] = 'L', if the ith domino has been pushed to the left,
// dominoes[i] = 'R', if the ith domino has been pushed to the right, and
// dominoes[i] = '.', if the ith domino has not been pushed.
// Return a string representing the final state.

// Constraints:
// - n == dominoes.length
// - 1 <= n <= 105
// - dominoes[i] is either 'L', 'R', or '.'.

struct Solution;
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let n = dominoes.len();
        let mut dominoes = dominoes.chars().collect::<Vec<_>>();

        for i in 0..n {
            if dominoes[i] == '.' {
                if i > 0 && dominoes[i - 1] == 'R' {
                    let mut idx = i;
                    while idx < n && dominoes[idx] == '.' {
                        idx += 1;
                    }
                    if idx == n {
                        dominoes[i..n].iter_mut().for_each(|d| *d = 'R');
                    } else if dominoes[idx] == 'R' {
                        dominoes[i..idx].iter_mut().for_each(|d| *d = 'R');
                    } else {
                      let diff = idx - i;
                      match diff % 2 {
                        0 => {
                          dominoes[i..(i +diff/2)].iter_mut().for_each(|d| *d = 'R');
                          dominoes[(i + diff/2)..idx].iter_mut().for_each(|d| *d = 'L');
                        },
                        _ => {
                          dominoes[i..(i + diff/2)].iter_mut().for_each(|d| *d = 'R');
                          dominoes[(i + diff/2 + 1)..idx].iter_mut().for_each(|d| *d = 'L');
                        }
                      }

                    }
                }
                if i < n - 1 && dominoes[i + 1] == 'L' {
                    let mut idx = i;
                    while idx >= 0 && dominoes[idx] == '.' {
                        if idx > 0 && dominoes[idx - 1] == 'R' {
                            break;
                        }
                        dominoes[idx] = 'L';
                        if idx == 0 {
                            break;
                        }
                        idx -= 1;
                    }
                }
            }
        }
        dominoes.iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let dominoes = "RR.L".to_string();
        let expected = "RR.L".to_string();
        assert_eq!(Solution::push_dominoes(dominoes), expected);
    }

    #[test]
    fn test_2() {
        let dominoes = ".L.R...LR..L..".to_string();
        let expected = "LL.RR.LLRRLL..".to_string();
        assert_eq!(Solution::push_dominoes(dominoes), expected);
    }

    #[test]
    fn test_3() {
        let dominoes = ".L.....LR..L..".to_string();
        let expected = "LLLLLLLLRRLL..".to_string();
        assert_eq!(Solution::push_dominoes(dominoes), expected);
    }

    #[test]
    fn test_4() {
        let dominoes = ".L.R...RLR..L..".to_string();
        let expected = "LL.RRRRRLRRLL..".to_string();
        assert_eq!(Solution::push_dominoes(dominoes), expected);
    }
}
