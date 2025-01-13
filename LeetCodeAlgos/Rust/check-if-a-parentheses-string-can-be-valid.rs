// A parentheses string is a non-empty string consisting only of '(' and ')'. It is valid if any of the following conditions is true:
//
// It is ().
// It can be written as AB (A concatenated with B), where A and B are valid parentheses strings.
// It can be written as (A), where A is a valid parentheses string.
// You are given a parentheses string s and a string locked, both of length n. locked is a binary string consisting only of '0's and '1's. For each index i of locked,
//
// If locked[i] is '1', you cannot change s[i].
// But if locked[i] is '0', you can change s[i] to either '(' or ')'.
// Return true if you can make s a valid parentheses string. Otherwise, return false.


struct Solution {}

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        if s.len() % 2 != 0 {
            return false;
        }
        let mut open: Vec<usize> = Vec::new();
        let mut unlocked: Vec<usize> = Vec::new();

        let res = s
            .as_bytes()
            .iter()
            .zip(locked.as_bytes().iter())
            .map(|(&c, &l)| (if c == '(' as u8 {true} else {false}, if l == '1' as u8 {true} else {false}))
            .enumerate()
            .map(|(i, (is_start, is_locked))| {
                if !is_locked {
                    unlocked.push(i);
                    true
                } else if is_start {
                    open.push(i);
                    true
                } else {
                    match (open.last(), unlocked.last()) {
                        (Some(&o), _) => {
                            open.pop();
                            true
                        },
                        (None, Some(&u)) => {
                            unlocked.pop();
                            true
                        },
                        _ => false,
                    }
                }
            })
            .all(|ok| ok);

        if !res {
            return res;
        }

        std::iter::successors(open.last().copied().zip(unlocked.last().copied()), |(o, u)| {
            if o < u {
                open.pop();
                unlocked.pop();
                open.last().copied().zip(unlocked.last().copied())
            } else {
                None
            }
        })
            .last();

        open.is_empty()
    }
}

// use std::collections::VecDeque;
// impl Solution {
//     pub fn can_be_valid(s: String, locked: String) -> bool {
//         // Odd length is instant fail for half of possible cases
//         if  s.len() % 2 == 1 { return false; }
//         // All '0' locked instant pass
//         else if i32::from_str_radix(&locked[..], 2) == Ok(0) { return true; }
//
//         let s: Vec<char> = s.chars().collect();
//         let locked: Vec<char> = locked.chars().collect();
//
//         let mut stack: VecDeque<(usize, i32)> = VecDeque::new();
//         let mut wildcards: VecDeque<usize> = VecDeque::new();
//
//         let mut closed_parens: i32 = 0;
//         let check_count = |closed: &mut i32, count: i32| -> Result<(), ()>{
//             if *closed - count > 2 {
//                 return Err(());
//             }
//             *closed -= *closed - count;
//             *closed += 1;
//             Ok(())
//         };
//         for idx in 0..s.len() {
//             println!("{}", locked[idx]);
//             match locked[idx] {
//                 '0' => {
//                     println!("Open -> {:?}", stack.back());
//                     wildcards.push_back(idx)
//                 }
//                 _ => {
//                     println!("Closed -> {}", s[idx]);
//                     match s[idx] {
//                         '(' => {
//                             println!("Opening parens {}", closed_parens);
//                             stack.push_back((idx, closed_parens));
//                         },
//                         _ => {
//                             // If no open pair and locked
//                             println!("Closing parens {:?}", stack.back());
//                             if stack.is_empty() && wildcards.is_empty() {
//                                 return false;
//                             }
//                             else if let Some(unlocked_idx) = wildcards.back() {
//                                 if *unlocked_idx < idx {
//                                     wildcards.pop_back();
//                                     break;
//                                 }
//                             }
//                             if let Err(_) = check_count(&mut closed_parens, stack.pop_back().unwrap().1) { return false; }
//                         }
//                     }
//                 }
//             }
//         }
//
//         while let Some((open_idx, _)) = stack.pop_back() {
//             if let Some(unlocked_idx) = wildcards.pop_back() {
//                 if unlocked_idx > open_idx {
//                     continue;
//                 } else {
//                     return false;
//                 }
//             } else {
//                 return false;
//             }
//         }
//         true
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1(){
        assert_eq!(
            Solution::can_be_valid(
                String::from("))()))"),
                String::from("010100")
            ),
            true
        );
    }

    #[test]
    fn test2(){
        assert_eq!(
            Solution::can_be_valid(
                String::from("()()"),
                String::from("0000")
            ),
            true
        );
    }

    #[test]
    fn test3(){
        assert_eq!(
            Solution::can_be_valid(
                String::from(")"),
                String::from("0")
            ),
            false
        );
    }

    #[test]
    fn test4(){
        assert_eq!(
            Solution::can_be_valid(
                String::from("((()()())())"),
                String::from("111111111111")
            ),
            false
        );
    }

    #[test]
    fn test5(){
        assert_eq!(
            Solution::can_be_valid(
                String::from("((()()()()()"),
                String::from("111111111111")
            ),
            false
        );
    }

}