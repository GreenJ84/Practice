// You are given an integer array prices where prices[i] is the price of a given stock on the ith day.

// On each day, you may decide to buy and/or sell the stock. You can only hold at most one share of the stock at any time. However, you can buy it then immediately sell it on the same day.

// Find and return the maximum profit you can achieve.

public class Solution {
    public int MaxProfit(int[] prices) {
        int profit = 0;
        for (int idx = 1; idx < prices.Length; idx++) {
          if (prices[idx] > prices[idx - 1]) {
            profit += prices[idx] - prices[idx - 1];
          }
        }
        return profit;
    }
}

public void TestMaxProfit(int testNum, int[] input, int expected) {
  Solution solution = new Solution();
  int result = solution.MaxProfit(input);

  Console.WriteLine($"Test {testNum}: {string.Join(", ", input)} -> {result} / {expected} ({result == expected})");
}

TestMaxProfit(1, new int[] { 7, 1, 5, 3, 6, 4 }, 7);
TestMaxProfit(2, new int[] { 1,2,3,4,5 }, 4);