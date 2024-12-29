import java.util.PriorityQueue;

public class KthLargestElementInStream {
  public static void main(String[] args) {
    KthLargestElementInStream stream = new KthLargestElementInStream(3, new int[]{4, 5, 8, 2});
    testLog(4, stream.add(3));
    testLog(5, stream.add(5));
    testLog(5, stream.add(10));
    testLog(8, stream.add(9));
    testLog(8, stream.add(4));

    stream = new KthLargestElementInStream(4, new int[]{7, 7, 7, 7, 8, 3});
    testLog(7, stream.add(2));
    testLog(7, stream.add(10));
    testLog(7, stream.add(9));
    testLog(8, stream.add(9));
  }
  public static void testLog(int expected, int actual){
    System.out.printf("%d == %d (%b)\n", expected, actual, expected == actual);
  }



  PriorityQueue<Integer> queue;
  int k;
  public KthLargestElementInStream(int k, int[] nums) {
    this.k = k;
    this.queue = new PriorityQueue<>(k, Integer::compareTo);
    for (int num : nums) {
      this.add(num);
    }
  }

  public int add(int val) {
    if (queue.size() >= this.k){
      if (queue.peek() > val) {
        return queue.peek();
      }
      queue.poll();
    }
    queue.offer(val);
    return queue.peek();
  }
}
