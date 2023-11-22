// Given two string arrays word1 and word2, return true if the two arrays represent the same string, and false otherwise.

// A string is represented by an array if the array elements concatenated in order forms the string.

class checkTwoStringArraysEquivelant{
  public boolean arrayStringsAreEqual(String[] word1, String[] word2) {
        String w1 = String.join("", word1);
        String w2 = String.join("", word2);

        if (w1.length() != w2.length()){
            return false;
        }
        for (int i = 0; i < w1.length(); i++){
            if (w1.charAt(i) != w2.charAt(i)){
                return false;
            }
        }
        return true;
    }

  public static void main(String[] args) {
    checkTwoStringArraysEquivelant s = new checkTwoStringArraysEquivelant();
    System.out.println(s.arrayStringsAreEqual(
      new String[]{"ab", "c"}, 
      new String[]{"a", "bc"}));
    System.out.println(s.arrayStringsAreEqual(
      new String[]{"a", "cb"}, 
      new String[]{"ab", "b"}));
    System.out.println(s.arrayStringsAreEqual(
      new String[]{"abc", "d", "defg"}, 
      new String[]{"abcddefg"}));
  }
}
