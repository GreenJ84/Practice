// You are given an integer array arr of length n that represents a permutation of the integers in the range [0, n - 1].

// We split arr into some number of chunks (i.e., partitions), and individually sort each chunk. After concatenating them, the result should equal the sorted array.

// Return the largest number of chunks we can make to sort the array.

public class MaxChunksToMakeSorted {
  public static void main(String[] args) {
    MaxChunksToMakeSorted obj = new MaxChunksToMakeSorted();

    System.out.println(obj.maxChunksToSorted(new int[]{4,3,2,1,0})); // 1
    System.out.println(obj.maxChunksToSorted(new int[]{1,0,2,3,4})); // 4
    System.out.println(obj.maxChunksToSorted(new int[]{8,1,2,3,4,9,5,6,7})); // 1
  }

  public int maxChunksToSorted(int[] arr) {
    int n = arr.length, chunks = 0;
    if (n == 1) return 1;

    int[] prefixMax = new int[n];
    prefixMax[0] = arr[0];
    for (int i = 1; i < n; i++) {
      prefixMax[i] = Math.max(prefixMax[i-1], arr[i]);
    }

    int[] suffixMin = new int[n];
    suffixMin[n-1] = arr[n-1];
    for (int i = n-2; i >= 0; i--) {
      suffixMin[i] = Math.min(suffixMin[i+1], arr[i]);
    }

    for (int i = 0; i < n; i++) {
      if ( prefixMax[i] == suffixMin[i] ||
        (i < n-1 && prefixMax[i] < suffixMin[i + 1]) ||
        i == n-1
      ) {chunks++;}
    }

    return chunks;
  }
}
