// Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.

public class Solution {
    public int StrStr(string haystack, string needle) {
        if (haystack.Length < needle.Length) return -1;
        if (haystack.Length == needle.Length) return haystack == needle;
        for (int start = 0; start < haystack.Length - needle.Length; start++)
        {
          if (haystack[start] == needle[0])
          {
            bool found = true;
            for (int run = 1; run < needle.Length; run++)
            {
              if (haystack[start + run] != needle[run])
              {
                found = false;
                break;
              }
            }
            if (found)
              return start;
          }
        }
        return -1;
    }
}