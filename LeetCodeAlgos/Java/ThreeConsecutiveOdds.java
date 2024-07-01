// Given an integer array arr, return true if there are three consecutive odd numbers in the array. Otherwise, return false.

public class ThreeConsecutiveOdds {
  public static void main(String[] args) {
    Solution s = new Solution();

    testThreeConsecutiveOdds(1, new int[]{2, 6, 4, 1}, false, s);
    testThreeConsecutiveOdds(2, new int[]{1,2,34,3,4,5,7,23,12}, true, s);
    testThreeConsecutiveOdds(3, new int[]{1, 2, 3, 4, 6}, false, s);
    testThreeConsecutiveOdds(4, new int[]{1, 2, 1, 3, 5, 6}, true, s);
  }

  private static void testThreeConsecutiveOdds(int testNum, int[] arr, boolean expected, Solution s){
    boolean result = s.threeConsecutiveOdds(arr);

    System.out.println(String.format(
      "Test %d: %b / %b (%s)",
      testNum,
      result,
      expected,
      result == expected? "PASS" : "FAIL"
    ));
  }
}

class Solution {
  public boolean threeConsecutiveOdds(int[] arr) {
      int window = 0;
      for (int i = 0; i < arr.length; i++) {
        if (arr[i] % 2 == 0){
          window = 0;
          continue;
        }
        window++;
        if (window == 3) return true;
      }
      return false;
  }
}

//!!!! Traditional For loop always beats on Memory due to underlying iterator implementation
// class Solution {
//   public boolean threeConsecutiveOdds(int[] arr) {
//       int window = 0;
//       for (int num : arr){
//         if (num % 2 == 0){
//           window = 0;
//           continue;
//         }
//         window++;
//         if (window == 3) return true;
//       }
//       return false;
//   }
// }