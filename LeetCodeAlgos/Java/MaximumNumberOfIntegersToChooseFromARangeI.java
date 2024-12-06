import java.util.HashSet;

class MaximumNumberOfIntegersToChooseFromARangeI {
  public static void main(String[] args) {
    MaximumNumberOfIntegersToChooseFromARangeI obj = new MaximumNumberOfIntegersToChooseFromARangeI();

    System.out.println(obj.maxCount(new int[]{1,6,5}, 5, 6));
    System.out.println(obj.maxCount(new int[]{1,2,3,4,5,6,7}, 8, 1));
    System.out.println(obj.maxCount(new int[]{11}, 7, 50));
  }
  public int maxCount(int[] banned, int n, int maxSum) {
      HashSet<Integer> bannedSet = new HashSet<>();
      for (int num : banned) {
          bannedSet.add(num);
      }
      int count = 0;
      int sum = 0;
      for (int i = 1; i <= n; i++) {
          if (!bannedSet.contains(i)) {
            sum += i;
            if (sum > maxSum) return count;
            count++;
          }
      }
      return count;
  }
}