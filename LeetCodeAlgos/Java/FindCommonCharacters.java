// Given a string array words, return an array of all characters that show up in all strings within the words (including duplicates). You may return the answer in any order.

import java.util.Map;
import java.util.List;
import java.util.HashMap;
import java.util.Iterator;
import java.util.ArrayList;

public class FindCommonCharacters {
  public static void main(String[] args) {
    Solution s = new Solution();

    testCommonChars(1, new String[]{"bella","label","roller"}, List.of("e","l","l"), s);
    testCommonChars(2, new String[]{"cool","lock","cook"}, List.of("c","o"), s);
  }

  private static void testCommonChars(int testNum, String[] words, List<String> expected, Solution s){
    List<String> result = s.commonChars(words);

    System.out.println(String.format(
      "Test %d: %s / %s (%b)",
      testNum,
      result,
      expected,
      result.equals(expected)
    ));
  }
}

class Solution {
  public List<String> commonChars(String[] words) {
      HashMap<Character, Integer> common = frequencyMap(words[0]);
      for (int idx = 0; idx < words.length; idx++) {
        HashMap<Character, Integer> newWord = frequencyMap(words[idx]);
        Iterator<Character> iter = common.keySet().iterator();

        while (iter.hasNext()) {
          Character ch = iter.next();
          if (newWord.containsKey(ch)) {
            common.put(ch, Math.min(common.get(ch), newWord.get(ch)));
          } else {
            iter.remove();
          }
        }
      }

      List<String> result = new ArrayList<>();
      for (Map.Entry<Character, Integer> entry: common.entrySet()) {
        for (int i = 0; i < entry.getValue(); i++) {
          result.add(entry.getKey().toString());
        }
      }

      return result;
  }

  private HashMap<Character, Integer> frequencyMap(String str){
    HashMap<Character, Integer> freq = new HashMap<>();
    for (Character c: str.toCharArray()){
      freq.put(c, freq.getOrDefault(c, 0) + 1);
    }
    return freq;
  }
}