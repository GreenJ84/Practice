// You are given an array prices where prices[i] is the price of a given stock on the ith day.
// You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.
// Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.

public class Solution {
    public int MaxProfit(int[] prices) {
      int profit = 0;
      int lowestPrice = prices[0];

      for (int idx = 1; idx < prices.Length; idx++) {
        if (prices[idx] > lowestPrice) {
          int diff = prices[idx] - lowestPrice;
          if (diff > profit){
            profit = diff;
          }
        } else if (prices[idx] < lowestPrice) {
          lowestPrice = prices[idx];
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

TestMaxProfit(1, new int[] { 7, 1, 5, 3, 6, 4 }, 5);
TestMaxProfit(2, new int[] { 7, 6, 4, 3, 1 }, 0);