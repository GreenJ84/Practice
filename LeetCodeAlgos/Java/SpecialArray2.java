// An array is considered special if every pair of its adjacent elements contains two numbers with different parity.

// You are given an array of integer nums and a 2D integer matrix queries, where for queries[i] = [fromi, toi] your task is to check that 
// subarray
//  nums[fromi..toi] is special or not.

// Return an array of booleans answer such that answer[i] is true if nums[fromi..toi] is special.

public class SpecialArray2 {
  public static void main(String[] args) {
    SpecialArray2 obj = new SpecialArray2();
    System.out.println(arrayToString(obj.isArraySpecial(
      new int[]{3,4,1,2,6},
      new int[][]{new int[]{0,4}}
    )));
    System.out.println(arrayToString(obj.isArraySpecial(
      new int[]{4,3,1,6},
      new int[][]{new int[]{0,2}, new int[]{2,3}}
    )));
  }

  public static String arrayToString(boolean[] array) {
    StringBuilder sb = new StringBuilder("[");
    for (int i = 0; i < array.length; i++) {
        sb.append(array[i]);
        if (i < array.length - 1) {
            sb.append(", ");
        }
    }
    sb.append("]");
    return sb.toString();
  }

  public boolean[] isArraySpecial(int[] nums, int[][] queries) {
    int n = nums.length;
    boolean[] answer = new boolean[queries.length];

    int[] parity = new int[n];
    for (int numIdx = 0; numIdx < n; numIdx++) {
      parity[numIdx] = nums[numIdx] % 2;
    }

    int[] parityDiff = new int[n];
    for (int numIdx = 1; numIdx < n; numIdx++) {
      parityDiff[numIdx] = Math.abs(parity[numIdx] - parity[numIdx - 1]);
    }

    int[] preComp = new int[n + 1];
    for (int numIdx = 1; numIdx <= n; numIdx++){
      preComp[numIdx] = preComp[numIdx - 1] + parityDiff[numIdx - 1];
    }

    for (int qIdx = 0; qIdx < queries.length; qIdx++) {
      int left = queries[qIdx][0];
      int right = queries[qIdx][1];
      if (left == right) {
        answer[qIdx] = true;
        continue;
      }
      int parityChanges = preComp[right + 1] - preComp[left + 1];
      answer[qIdx] = parityChanges == (right - left);
    }
    return answer;
  }

//! TOO SLOW
  // public boolean[] isArraySpecial(int[] nums, int[][] queries) {
  //   boolean[] answer = new boolean[queries.length];
  //   int left, right, lastParity;

  //   for (int qIdx = 0; qIdx < queries.length; qIdx++) {
  //     left = queries[qIdx][0];
  //     right = queries[qIdx][1];
  //     if (left == right) {
  //       answer[qIdx] = true;
  //       continue;
  //     }
  //     lastParity =  nums[left] % 2;
  //     for (int numIdx = left + 1; numIdx <= right; numIdx++) {
  //       int parity = nums[numIdx] % 2;
  //       if (parity == lastParity) {
  //         answer[qIdx] = false;
  //         break;
  //       }
  //       if (numIdx == right){
  //         answer[qIdx] = true;
  //         break;
  //       }
  //       lastParity = parity;
  //     }
  //   }
  //   return answer;
  // }
}
