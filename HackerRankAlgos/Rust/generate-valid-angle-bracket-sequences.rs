// Given n, return all valid sequences of n pairs of '<' and '>' with proper nesting.


// fn generateAngleBracketSequences(n: i32) -> Vec<String> {
fn generate_angle_bracket_sequences(n: i32) -> Vec<String> {
    let mut ans = Vec::<String>::new();
    finish_sequence(&n, 1, String::from("<"), &mut ans);
    ans
}

fn finish_sequence(n: &i32, offset: i32, partial: String, finished: &mut Vec<String>){
  if offset == 0 && partial.len() == 2 * (*n as usize) {
    finished.push(partial.to_string());
    return;
  }
  if 2 * (*n as usize) - partial.len() > offset as usize {
    finish_sequence(n, offset + 1, partial.clone() + "<", finished);
  }
  if offset > 0 {
    finish_sequence(n, offset - 1, partial.clone() + ">", finished);
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 1;
        let expected = vec!["<>".to_string()];
        assert_eq!(generate_angle_bracket_sequences(n), expected);
    }

    #[test]
    fn test2() {
        let n = 2;
        let expected = vec!["<<>>".to_string(), "<><>".to_string()];
        assert_eq!(generate_angle_bracket_sequences(n), expected);
    }

    #[test]
    fn test3() {
        let n = 3;
        let expected = vec!["<<<>>>".to_string(), "<<><>>".to_string(), "<<>><>".to_string(), "<><<>>".to_string(), "<><><>".to_string()];
        assert_eq!(generate_angle_bracket_sequences(n), expected);
    }
}
