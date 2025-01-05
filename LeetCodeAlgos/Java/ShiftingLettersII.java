// You are given a string s of lowercase English letters and a 2D integer array shifts where shifts[i] = [starti, endi, directioni]. For every i, shift the characters in s from the index starti to the index endi (inclusive) forward if directioni = 1, or shift the characters backward if directioni = 0.

// Shifting a character forward means replacing it with the next letter in the alphabet (wrapping around so that 'z' becomes 'a'). Similarly, shifting a character backward means replacing it with the previous letter in the alphabet (wrapping around so that 'a' becomes 'z').

// Return the final string after all such shifts to s are applied.



public class ShiftingLettersII {
  public static void main(String[] args) {
    ShiftingLettersII obj = new ShiftingLettersII();

    System.out.println(obj.shiftingLetters("abc", new int[][]{{0,1,0}, {1,2,1}, {0,2,1}}));
    System.out.println(obj.shiftingLetters("dztz", new int[][]{{0,0,0}, {1,1,1}}));
  }

  public String shiftingLetters(String s, int[][] shifts) {
    int[] track = new int[s.length() + 1];
    for (int[] shift : shifts) {
      int start = shift[0], end = shift[1], direction = shift[2];
      if (direction == 0) {
        track[start]--;
        track[end + 1]++;
      } else {
        track[start]++;
        track[end + 1]--;
      }
    }

    int totalShifts = 0;
    StringBuilder sb = new StringBuilder();
    for (int i = 0; i < s.length(); i++) {
      char c = s.charAt(i);
      totalShifts += track[i];
      int newChar = c + (totalShifts % 26);
      if (newChar > 'z') {
        newChar = newChar - 'z' + 'a' - 1;
      } else if (newChar < 'a') {
        newChar = newChar + 'z' - 'a' + 1;
      }
      sb.append((char) newChar);
    }
    return sb.toString();
  }

  // public String shiftingLetters(String s, int[][] shifts) {
  //     int[] track = new int[s.length()];
  //     for (int[] shift : shifts) {
  //       int start = shift[0], end = shift[1], direction = shift[2];
  //       for (int i = start; i <= end; i++) {
  //         track[i] += direction == 0 ? -1 : 1;
  //       }
  //     }
  //     StringBuilder sb = new StringBuilder();
  //     for (int i = 0; i < s.length(); i++) {
  //       char c = s.charAt(i);
  //       int shift = track[i];
  //       int newChar = c + (shift % 26);
  //       if (newChar > 'z') {
  //         newChar = newChar - 'z' + 'a' - 1;
  //       } else if (newChar < 'a') {
  //         newChar = newChar + 'z' - 'a' + 1;
  //       }
  //       sb.append((char) newChar);
  //     }
  //     return sb.toString();
  // }
}
