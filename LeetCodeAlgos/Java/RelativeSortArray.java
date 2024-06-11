// Given two arrays arr1 and arr2, the elements of arr2 are distinct, and all elements in arr2 are also in arr1.

// Sort the elements of arr1 such that the relative ordering of items in arr1 are the same as in arr2. Elements that do not appear in arr2 should be placed at the end of arr1 in ascending order.

import java.util.*;
import java.util.stream.*;

public class RelativeSortArray {
  public static void main(String[] args) {
    Solution solution = new Solution();

    testRelativeSortArray(1,
      new int[]{2,3,1,3,2,4,6,7,9,2,19},
      new int[]{2,1,4,3,9,6},
      new int[]{2,2,2,1,4,3,3,9,6,7,19},
      solution
    );
    testRelativeSortArray(2,
      new int[]{28,6,22,8,44,17},
      new int[]{22,28,8,6},
      new int[]{22,28,8,6,17,44},
      solution
    );
  }

  private static void testRelativeSortArray(int testNum, int[] arr1, int[] arr2, int[] expected, Solution s){
    int[] result = s.relativeSortArray(arr1, arr2);

    System.out.println(String.format(
      "Test %d:\n\t%s\n\t%s \n(%s)\n",
      testNum,
      Arrays.toString(result),
      Arrays.toString(expected),
      Arrays.equals(result, expected)? "PASS" : "FAIL"
    ));
  }
}

class IndexComparator implements Comparator<Integer>{
  HashMap<Integer, Integer> sortMap;
  IndexComparator(int[] keyArray) {
    this.sortMap = new HashMap<>();
    for (int i = 0; i < keyArray.length; i++) {
      this.sortMap.put(keyArray[i], i);
    }
  }
  @Override
  public int compare(Integer a, Integer b){
    return this.sortMap.getOrDefault(a, 1000 + a) - this.sortMap.getOrDefault(b, 1000 + b);
  }
}
class Solution {
  public int[] relativeSortArray(int[] arr1, int[] arr2) {
    List<Integer> result = Arrays.stream(arr1).boxed().collect(Collectors.toList());
    Collections.sort(
      result,
      new IndexComparator(arr2)
    );
    return result.stream().mapToInt(Integer::intValue).toArray();
  }
}