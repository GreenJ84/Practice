// In English, we have a concept called root, which can be followed by some other word to form another longer word - let's call this word derivative. For example, when the root "help" is followed by the word "ful", we can form a derivative "helpful".

// Given a dictionary consisting of many roots and a sentence consisting of words separated by spaces, replace all the derivatives in the sentence with the root forming it. If a derivative can be replaced by more than one root, replace it with the root that has the shortest length.

// Return the sentence after the replacement.

import java.util.*;

public class ReplaceWords {
  public static void main(String[] args) {
    Solution solution = new Solution();

    testReplaceWords(1, List.of("cat","bat","rat"), "the cattle was rattled by the battery", "the cat was rat by the bat", solution);
    testReplaceWords(2,  List.of("a","b","c"), "aadsfasf absbs bbab cadsfafs", "a a b c", solution);
  }

  private static void testReplaceWords(int testNum, List<String> dictionary, String sentence, String expected, Solution s) {
    String result = s.replaceWords(dictionary, sentence);

    System.out.println(String.format(
      "Test %d: %s / %s (%s)",
      testNum,
      result,
      expected,
      result.equals(expected) ? "PASS" : "FAIL"
    ));
  }
}

class Trie {
  public Trie[] children = new Trie[26];
  private boolean wordEnd;
  Trie(){
    this.wordEnd = false;
    for (int i = 0; i < 26; i++) {
      this.children[i] = null;
    }
  }
  public void insert(String key){
    Trie current = this;
    for (Character c : key.toCharArray()){
      if (current.children[c - 'a'] == null){
        current.children[c - 'a'] = new Trie();
      }
      current = current.children[c - 'a'];
    }
    current.wordEnd = true;
  }

  public boolean search(String word){
    Trie current = this;
    for (Character c : word.toCharArray()){
      if (current.children[c - 'a'] == null){
        return false;
      }
      current = current.children[c - 'a'];
    }
    return current.wordEnd;
  }

  public String firstKey(String word){
    Trie current = this.children[word.charAt(0) - 'a'];
    // No root
    if (current == null) return word;

    int idx = 1;
    while (idx < word.length()){
      // Return shortest root found
      if (current.wordEnd){
        return word.substring(0, idx);
      }
      // Stop looking is word trace broken
      else if (current.children[word.charAt(idx) - 'a'] == null){
        break;
      }
      // Move down Trie until an end is found or search excited
      current = current.children[word.charAt(idx++) - 'a'];
    }
    // Return full word if no root matches pattern
    return word;
  }

}

class Solution {
  public String replaceWords(List<String> dictionary, String sentence) {
    Trie root = new Trie();
    for (String word : dictionary){
      root.insert(word);
    }

    String[] words = sentence.split(" ");
    for (int idx = 0; idx < words.length; idx++) {
      words[idx] = root.firstKey(words[idx].trim());
    }
    return String.join(" ", words);
  }
}