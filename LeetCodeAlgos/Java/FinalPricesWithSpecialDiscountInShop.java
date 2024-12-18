// You are given an integer array prices where prices[i] is the price of the ith item in a shop.

// There is a special discount for items in the shop. If you buy the ith item, then you will receive a discount equivalent to prices[j] where j is the minimum index such that j > i and prices[j] <= prices[i]. Otherwise, you will not receive any discount at all.

// Return an integer array answer where answer[i] is the final price you will pay for the ith item of the shop, considering the special discount.

import java.util.*;

public class FinalPricesWithSpecialDiscountInShop {
  public static void main(String[] args) {
    FinalPricesWithSpecialDiscountInShop obj = new FinalPricesWithSpecialDiscountInShop();

    System.out.println(Arrays.toString(obj.finalPrices(new int[]{8,4,6,2,3})));
    System.out.println(Arrays.toString(obj.finalPrices(new int[]{1,2,3,4,5})));
    System.out.println(Arrays.toString(obj.finalPrices(new int[]{10,1,1,6})));
  }

  public int[] finalPrices(int[] prices) {
      int n = prices.length;
      Stack<Integer> stack = new Stack<>();

      for (int idx = 0; idx < n; idx++){
        while (!stack.isEmpty() && prices[stack.peek()] >= prices[idx]){
          prices[stack.pop()] -= prices[idx];
        }
        stack.add(idx);
      }
      return prices;
  }
}
