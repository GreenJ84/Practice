// You have a lock in front of you with 4 circular wheels. Each wheel has 10 slots: '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'. The wheels can rotate freely and wrap around: for example we can turn '9' to be '0', or '0' to be '9'. Each move consists of turning one wheel one slot.
// The lock initially starts at '0000', a string representing the state of the 4 wheels.
// You are given a list of deadends dead ends, meaning if the lock displays any of these codes, the wheels of the lock will stop turning and you will be unable to open it.
// Given a target representing the value of the wheels that will unlock the lock, return the minimum total number of turns required to open the lock, or -1 if it is impossible.

import java.util.*;

public class OpenTheLock {
  public static void main(String[] args) {
    System.out.println((int)'0' - 48);
    System.out.println((int)'2');
    System.out.println((int)'4');
    System.out.println((int)'3');
    System.out.println((int)'6');
    System.out.println((int)'5');
    System.out.println((int)'7');
  }
}

class Solution {
  public int openLock(String[] deadends, String target) {
      // Create a dead set for easy lookup
      Set<String> dead = new HashSet<>(Arrays.asList(deadends));
      // Easy finish edge cases
      if (dead.contains("0000") || dead.contains(target)) {
        return -1;
      } else if (target.equals("0000")) {
        return 0;
      }
      // Create set to keep track of seen states
      HashMap<String, Integer> seen = new HashMap<>();
      // Create a queue to keep track of successful combo moves
      Queue<String> q = new LinkedList<>();
      // Initialize
      q.add("0000");
      seen.put("0000", 0);
      while (!q.isEmpty()) {
        String curr = q.poll();
        int count = seen.get(curr) + 1;
        for (int idx = 0; idx < 4; idx++) {
          for (int adjust = -1; adjust <= 1; adjust += 2) {
            int currChar = (curr.charAt(idx) - '0' + adjust + 10) % 10;
            String next = curr.substring(0, idx) + currChar + curr.substring(idx + 1);
            if (!seen.containsKey(next) && !dead.contains(next)) {
              if (next.equals(target)) {
                return count;
              }
              seen.put(next.toString(), count);
              q.offer(next.toString());
            }
          }
        }
      }
      return -1;
  }
}