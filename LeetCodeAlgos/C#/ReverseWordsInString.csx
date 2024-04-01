// Given an input string s, reverse the order of the words.

// A word is defined as a sequence of non-space characters. The words in s will be separated by at least one space.

// Return a string of the words in reverse order concatenated by a single space.

// Note that s may contain leading or trailing spaces or multiple spaces between two words. The returned string should only have a single space separating the words. Do not include any extra spaces.

public class Solution {
    public string ReverseWords(string s) {
        Stack<string> hold = new Stack<string>();
        string word = "";
        // Find and add all words to stack
        for (int i = 0; i < s.Length; i++)
        {
            if (s[i] == ' ')
            {
                if (word.Length > 0)
                {
                    hold.Push(word);
                }
                word = "";
            }
            else
            {
                word += s[i];
            }
        }
        if (word.Length > 0)
        {
            hold.Push(word);
        }
        // Reverse all words in stack
        return string.Join(" ", hold);
    }
}

