import java.util.HashMap;
import java.util.Map.Entry;

public class FindLongestSpecialSubstringThatOccursThriceI {
  public static void main(String[] args) {
    FindLongestSpecialSubstringThatOccursThriceI solution = new FindLongestSpecialSubstringThatOccursThriceI();
    System.out.println(solution.maximumLength("aaaa")); // Output: 2
    System.out.println(solution.maximumLength("abcdef")); // Output: -1
    System.out.println(solution.maximumLength("abcaba")); // Output: 1
  }


  public int maximumLength(String s) {
    HashMap<String, Integer> count = new HashMap<>();

    for (int idx = 0; idx < s.length(); idx++) {
      int subIdx = idx;
      while (subIdx < s.length() && s.charAt(subIdx) == s.charAt(idx)) {
        String substring = s.substring(idx, subIdx + 1);
        count.put(substring, count.getOrDefault(substring, 0) + 1);
        subIdx++;
      }
    }

    int max = -1;
    for (Entry<String, Integer> entry : count.entrySet()){
      if (entry.getValue() >= 3 && entry.getKey().length() > max){
        max = entry.getKey().length();
      }
    }
    return max;
  }
}
