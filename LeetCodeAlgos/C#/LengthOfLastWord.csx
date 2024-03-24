// Given a string s consisting of words and spaces, return the length of the last word in the string.

// A word is a maximal 
// substring
//  consisting of non-space characters only.

public class Solution {
    public int LengthOfLastWord(string s) {
        bool reset = false;
        int len = 0;
        foreach (char ch in s){
            if (ch == ' '){
                reset = true;
            } else {
                if (reset){
                    len = 0;
                    reset = false;
                }
                len += 1;
            }
        }
        return len;
    }
}