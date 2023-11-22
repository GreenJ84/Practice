// You are given a 0-indexed integer array candies. Each element in the array denotes a pile of candies of size candies[i]. You can divide each pile into any number of sub piles, but you cannot merge two piles together.

// You are also given an integer k. You should allocate piles of candies to k children such that each child gets the same number of candies. Each child can take at most one pile of candies and some piles of candies may go unused.

// Return the maximum number of candies each child can get.

class Solution {
    public int maximumCandies(int[] candies, long k) {
        int minValue = Integer.MAX_VALUE;
        long totalCandies = 0;
        for (int pile: candies) {
          if (pile < minValue) {
            minValue = pile;
          }
          totalCandies += pile;
        }
        // If not enough total candies for kids, noone gets any
        if (totalCandies < k) {
          return 0;
        }
        // If total candies is equal to kids, everyone gets 1
        else if (totalCandies == k){
          return 1;
        }
        else {
          minValue = 1;
          int maxValue = (int)(totalCandies / k);
          int ans = 1;
          while (minValue <= maxValue) {
            int midValue = minValue + (maxValue - minValue) / 2;
            System.out.println(minValue + ", " + midValue + ", " + maxValue);
            if (isPossible(candies, midValue, k)) {
              ans = midValue;
              minValue = midValue + 1;
            }
            else {
              maxValue = midValue - 1;
            }
          }
          return ans;
        }
    }

    private boolean isPossible(int[] candies, int midValue, long k) {
      int totalDistro = 0;
      for (int pile: candies) {
        totalDistro += pile / midValue;
      }
      return totalDistro >= k;
    }
}

class MaximumCandiesToKChildren {
  public static void main(String[] args) {
    Solution solution = new Solution();
    int test1 = solution.maximumCandies(new int[]{5, 8, 6}, 3);
    System.out.println(test1);
    assert test1 == 5;

    int test2 = solution.maximumCandies(new int[]{2,5}, 11);
    System.out.println(test2);
    assert test2 == 5;

    int test3 = solution.maximumCandies(new int[]{4,7,5}, 4);
    System.out.println(test3);
    assert test3 == 3;
  }
}