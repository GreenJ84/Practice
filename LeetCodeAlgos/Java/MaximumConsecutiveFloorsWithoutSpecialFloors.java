// Alice manages a company and has rented some floors of a building as office space. Alice has decided some of these floors should be special floors, used for relaxation only.

// You are given two integers bottom and top, which denote that Alice has rented all the floors from bottom to top (inclusive). You are also given the integer array special, where special[i] denotes a special floor that Alice has designated for relaxation.

// Return the maximum number of consecutive floors without a special floor.

import java.util.*;

public class MaximumConsecutiveFloorsWithoutSpecialFloors {
  public static void main(String[] args) {
    MaximumConsecutiveFloorsWithoutSpecialFloors obj = new MaximumConsecutiveFloorsWithoutSpecialFloors();

    System.out.println(obj.maxConsecutive(2, 9, new int[]{4, 6}));
    System.out.println(obj.maxConsecutive(6, 8, new int[]{7,6,8}));
  }

  // public int maxConsecutive(int bottom, int top, int[] special) {
  //   HashSet<Integer> set = new HashSet<>();
  //   for (int floor : special){
  //     set.add(floor);
  //   }

  //   int max = 0, run = 0;
  //   for (int floor = bottom; floor <= top; floor++){
  //     if (set.contains(floor)) {run = 0; continue;}
  //     run++;
  //     if (run > max) max = run;
  //   }
  //   return max;
  // }

  public int maxConsecutive(int bottom, int top, int[] special) {
    Arrays.sort(special);
    int max = special[0] - bottom;

    for (int i = 1; i < special.length; i++) {
        max = Math.max(max, special[i] - special[i - 1] - 1);
    }
    max = Math.max(max, top - special[special.length - 1]);

    return max;
  }
}
