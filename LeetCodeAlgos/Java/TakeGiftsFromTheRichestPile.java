// You are given an integer array gifts denoting the number of gifts in various piles. Every second, you do the following:

// Choose the pile with the maximum number of gifts.
// If there is more than one pile with the maximum number of gifts, choose any.
// Leave behind the floor of the square root of the number of gifts in the pile. Take the rest of the gifts.
// Return the number of gifts remaining after k seconds.

import java.util.Collections;
import java.util.PriorityQueue;

public class TakeGiftsFromTheRichestPile {
  public static void main(String[] args) {
    TakeGiftsFromTheRichestPile obj = new TakeGiftsFromTheRichestPile();

    System.out.println(obj.pickGifts(new int[]{25,64,9,4,100}, 4));
    System.out.println(obj.pickGifts(new int[]{1,1,1,1}, 4));
  }
  
  public long pickGifts(int[] gifts, int k) {
      PriorityQueue<Integer> maxHeap = new PriorityQueue<>(Collections.reverseOrder());
      for (int gift : gifts) {
          maxHeap.offer(gift);
      }
      while (k > 0 && !maxHeap.isEmpty()) {
          int pileSize = maxHeap.poll();
          int remainingGifts = (int) Math.sqrt(pileSize);
          maxHeap.offer(remainingGifts);
          k--;
      }
      
      return maxHeap.stream().mapToLong(Integer::longValue).sum();
  }
}
