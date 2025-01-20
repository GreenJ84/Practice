import java.util.Arrays;
import java.util.HashMap;

public class FirstCompletelyPaintedRowOrColumn {
  public static void main(String[] args) {
    FirstCompletelyPaintedRowOrColumn obj = new FirstCompletelyPaintedRowOrColumn();

    System.out.println(obj.firstCompleteIndex(
      new int[]{1,3,4,2},
      new int[][]{{1,4},{2,3}}
    ));
    System.out.println(obj.firstCompleteIndex(
      new int[]{2,8,7,4,1,3,5,6,9},
      new int[][]{{3,2,5},{1,4,6},{8,7,9}}
    ));
  }

  public int firstCompleteIndex(int[] arr, int[][] mat) {
    int n = mat.length;
    int m = mat[0].length;

    HashMap<Integer, Integer> valueIdxMap = new HashMap<Integer, Integer>();
    for (int i = 0; i < arr.length; i++) {
      valueIdxMap.put(arr[i], i);
    }

    int[] row_max = new int[m];
    Arrays.fill(row_max, -1);
    int[] col_max = new int[n];
    Arrays.fill(col_max, -1);
    for (int row = 0; row < n; row++) {
      for (int col = 0; col < m; col++) {
        int value = valueIdxMap.get(mat[row][col]);
        row_max[row] = Math.max(row_max[row], value);
        col_max[col] = Math.max(col_max[col], value);
      }
    }

    int min = Integer.MAX_VALUE;
    for (int val : row_max){
      if (val < min && val >= 0) {
        min = val;
      }
    }
    for (int val : col_max){
      if (val < min && val >= 0) {
        min = val;
      }
    }

    return min;
  }
}
