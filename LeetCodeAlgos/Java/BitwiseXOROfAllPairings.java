// You are given two 0-indexed arrays, nums1 and nums2, consisting of non-negative integers. There exists another array, nums3, which contains the bitwise XOR of all pairings of integers between nums1 and nums2 (every integer in nums1 is paired with every integer in nums2 exactly once).

// Return the bitwise XOR of all integers in nums3.

// import java.util.HashMap;

public class BitwiseXOROfAllPairings {
  public static void main(String[] args) {
    BitwiseXOROfAllPairings obj = new BitwiseXOROfAllPairings();

    System.out.println(obj.xorAllNums(new int[]{2, 1, 3}, new int[]{10, 2, 5, 0}));

    System.out.println(obj.xorAllNums(new int[]{1, 2}, new int[]{3, 4}));
  }

  public int xorAllNums(int[] nums1, int[] nums2) {
    int xor1 = 0;
    for (int i = 0; i < nums1.length; i++) {
      xor1 ^= nums1[i];
    }
    int xor2 = 0;
    for (int j = 0; j < nums2.length; j++) {
      xor2 ^= nums2[j];
    }
    return (nums1.length % 2 * xor2) ^ (nums2.length % 2 * xor1);
  }
  //! Exceeds time limit because of O(n * m)
  //! Associative and commutative properties of XOR allows for optimization
  // public int xorAllNums(int[] nums1, int[] nums2) {
  //     int xor = 0;
  //     HashMap<String, Integer> cache = new HashMap<>();
  //     for (int i = 0; i < nums1.length; i++) {
  //       for (int j = 0; j < nums2.length; j++) {
  //         String key = String.format("&d:&d", nums1[i], nums2[j]);
  //         if (cache.containsKey(key)) {
  //           xor ^= cache.get(key);
  //         } else {
  //           int val = nums1[i] ^ nums2[j];
  //           xor ^= val;
  //           cache.put(key, val);
  //         }
  //         xor ^= nums1[i] ^ nums2[j];
  //       }
  //     }
  //     return xor;
  // }
}