public class CheckIfWordOccursAsPrefixOfAnyWordInSentence {
  public int isPrefixOfWord(String sentence, String searchWord) {
    int len = searchWord.length();
    int wordIdx = 0;
    String[] words = sentence.trim().split(" ");
    for (String word : words){
        ++wordIdx;
        if (word.length() < len){ continue; }
        for (int idx = 0; idx < len; idx++){
            if (searchWord.charAt(idx) != word.charAt(idx)) break;
            if (idx == len - 1) return wordIdx;
        }
    }
    return -1;
}
}
