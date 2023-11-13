// A pangram is a sentence where every letter of the English alphabet appears at least once.

// Given a string sentence containing only lowercase English letters, return true if sentence is a pangram, or false otherwise.

class Solution {
  public boolean checkIfPangram(String sentence) {
    if (sentence == null || sentence.length() < 26) {
      return false;
    }
    for (char alpha = 'a'; alpha <= 'z'; alpha++) {
      if (sentence.indexOf(alpha) == -1) {
        return false;
      }
    }
    return true;
  }
}

// import java.util.HashSet;
// class Solution {
//   public boolean checkIfPangram(String sentence) {
//     HashSet<Character> alpha = new HashSet<Character>();
//     for (int i = 97; i < 123; i++){
//         alpha.add((char) i);
//     }
//     for (char ch: sentence.toCharArray()){
//         if (alpha.contains(ch)){
//             alpha.remove(ch);
//         }
//     }
//     return alpha.size() == 0;
//   }
// }

class CheckIfSentenceIsPangram {
  public static void main(String[] args) {
    Solution s = new Solution();
    System.out.println(s.checkIfPangram("thequickbrownfoxjumpsoverthelazydog"));
    System.out.println(s.checkIfPangram("leetcode"));
  }
}