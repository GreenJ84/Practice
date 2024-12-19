import java.util.*;

public class ConstructStringWithRepeatLimit {
  public static void main(String[] args) {
    ConstructStringWithRepeatLimit obj = new ConstructStringWithRepeatLimit();

    System.out.println(obj.repeatLimitedString("cczazcc", 3));
    System.out.println(obj.repeatLimitedString("aababab", 2));
  }

  public String repeatLimitedString(String s, int repeatLimit) {
      Map<Character, Integer> map = new HashMap<>();
      for (char c : s.toCharArray()) {
          map.put(c, map.getOrDefault(c, 0) + 1);
      }

      PriorityQueue<Map.Entry<Character, Integer>> queue = new PriorityQueue<>((a, b) -> {
        return (int) b.getKey() - a.getKey();
      });
      for (Map.Entry<Character, Integer> entry : map.entrySet()) {
          queue.offer(entry);
      }

      StringBuilder sb = new StringBuilder();
      while (!queue.isEmpty()) {
        Map.Entry<Character, Integer> latest = queue.poll();
        int count = 0;
        while (count < latest.getValue() && count < repeatLimit) {
          sb.append(latest.getKey());
          count++;
        }
        if (latest.getValue() - count <= 0) {
          continue;
        }
        if (!queue.isEmpty()) {
          Map.Entry<Character, Integer> next = queue.poll();
          sb.append(next.getKey());
          if (next.getValue() - 1 > 0) queue.add(Map.entry(next.getKey(), next.getValue() - 1));
        } else {
          return sb.toString();
        }
        queue.add(Map.entry(latest.getKey(), latest.getValue() - count));
      }
      return sb.toString();
  }
}
