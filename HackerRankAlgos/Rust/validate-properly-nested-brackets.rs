// Validate Properly Nested Brackets
// Given a string, check if all brackets ('()', '{}', '[]') are properly matched and nested. Return 1 if valid, otherwise return 0.

// fn areBracketsProperlyMatched(code_snippet: &str) -> bool {
fn are_brackets_properly_matched(code_snippet: &str) -> bool {
  let mut stack = Vec::<char>::new();
  for ch in code_snippet.chars() {
    match ch {
      '{' | '[' | '(' => {
        stack.push(ch);
      },
      '}' | ']' | ')' => {
        if stack.is_empty() {
          return false;
        }
        match (stack.pop().unwrap(), ch){
          ('{', '}') | ('[', ']') | ('(', ')') => {},
          (_, _) => {
            return false;
          }
        }
      },
      _ => {}
    }
  }
  stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let code_snippet = "if (a[0] > b[1]) { doSomething(); }";
        assert_eq!(are_brackets_properly_matched(code_snippet), true);
    }

    #[test]
    fn test2() {
        let code_snippet = "if (a[0] > b[1]) { doSomething(); ";
        assert_eq!(are_brackets_properly_matched(code_snippet), false);
    }

    #[test]
    fn test3() {
        let code_snippet = "if (a[0] > b[1]) { doSomething(); }";
        assert_eq!(are_brackets_properly_matched(code_snippet), true);
    }
}
