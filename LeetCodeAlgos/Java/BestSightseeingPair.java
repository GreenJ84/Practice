// You are given an integer array values where values[i] represents the value of the ith sightseeing spot. Two sightseeing spots i and j have a distance j - i between them.

// The score of a pair (i < j) of sightseeing spots is values[i] + values[j] + i - j: the sum of the values of the sightseeing spots, minus the distance between them.

// Return the maximum score of a pair of sightseeing spots.

public class BestSightseeingPair {
  public static void main(String[] args) {
    BestSightseeingPair obj = new BestSightseeingPair();

    System.out.println(obj.maxScoreSightseeingPair(new int[]{8,1,5,2,6}));
    System.out.println(obj.maxScoreSightseeingPair(new int[]{1,2}));
    System.out.println(obj.maxScoreSightseeingPair(new int[]{1,3,5}));
  }

  public int maxScoreSightseeingPair(int[] values) {
    int n = values.length;
    if (n == 2) return values[0] + values[1] - 1;
    int maxScore = Integer.MIN_VALUE, left = 0;
    for (int right = 1; right < n; right++) {
      maxScore = Math.max(maxScore, values[left] + values[right] - right + left);
      if (right + values[right] >= left + values[left]) left = right;
    }
    return maxScore;
  }
}
