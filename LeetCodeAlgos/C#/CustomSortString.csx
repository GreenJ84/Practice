#!/usr/bin/env dotnet-script
using System;
using System.Collections.Generic;
using System.Linq;

public class Solution {
  // Your algorithm code
  public string CustomSortString(string order, string s)  {
      char[] orderArr = [.. order];
      Dictionary<char, int> track = new Dictionary<char, int>();
      StringBuilder result = new StringBuilder();

      foreach (char ch in s) {
        if (!orderArr.Contains(ch) || ch == orderArr[0]) {
          result.Append(ch);
        } else {
          track[ch] = track.ContainsKey(ch) ? track[ch] + 1 : 1;
        }
      }
      for (int idx = 1; idx < orderArr.Length; idx++) {
        if (track.ContainsKey(orderArr[idx])) {
          for (int count = 0; count < track[orderArr[idx]]; count++) {
            result.Append(orderArr[idx]);
          }
        }
      }
      return result.ToString();
  }
}


public static void TestCustomSortString(string[] inputs, string expect, int testNumber) {
    Solution solution = new Solution();
    string result =  solution.CustomSortString(inputs[0], inputs[1]);
    Console.WriteLine($"Test {testNumber}: {inputs[0]} => {result}({expect})");
}


// Run tests
TestCustomSortString(["cba", "abcd"], "cbad", 1);
TestCustomSortString(["bcafg", "abcd"], "bcad", 2);