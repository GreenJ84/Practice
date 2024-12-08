// You are given a 0-indexed 2D integer array of events where events[i] = [startTimei, endTimei, valuei]. The ith event starts at startTimei and ends at endTimei, and if you attend this event, you will receive a value of valuei. You can choose at most two non-overlapping events to attend such that the sum of their values is maximized.

// Return this maximum sum.

// Note that the start time and end time is inclusive: that is, you cannot attend two events where one of them starts and the other ends at the same time. More specifically, if you attend an event with end time t, the next event must start at or after t + 1.

import java.util.Arrays;
import java.util.PriorityQueue;

public class TwoBestNonOverlappingEvents {
  public static void main(String[] args) {
    TwoBestNonOverlappingEvents obj = new TwoBestNonOverlappingEvents();

    System.out.println(obj.maxTwoEvents(new int[][]{
      new int[]{1,3,2},
      new int[]{4,5,2},
      new int[]{2,4,3}
    }));
    System.out.println(obj.maxTwoEvents(new int[][]{
      new int[]{1,3,2},
      new int[]{4,5,2},
      new int[]{1,5,5}
    }));
    System.out.println(obj.maxTwoEvents(new int[][]{
      new int[]{1,5,3},
      new int[]{1,5,1},
      new int[]{6,6,5}
    }));
  }

    public int maxTwoEvents(int[][] events) {
    Arrays.sort(events, (a, b) -> Integer.compare(a[0], b[0]));
    PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> Integer.compare(a[0], b[0]));

    int max = 0, res = 0;
    for (int[] event : events) {
      int start = event[0], end = event[1], value = event[2];

      while (!pq.isEmpty() && pq.peek()[0] < start) {
        max = Math.max(max, pq.poll()[1]);
      };
      res = Math.max(max + value, res);
      pq.offer(new int[]{end, value});
    }
    return res;
  }

  //! Too Slow
  // public int binarySearch(int[][] events, int startTime){
  //   int left = 0;
  //   int right = events.length - 1;
  //   if (events[right][0] < startTime) return ++right;
  //   while (left < right) {
  //     int mid = (left + right) / 2;
  //     if (events[mid][0] >= startTime) {
  //       right = mid;
  //     } else {
  //       left = mid + 1;
  //     }
  //   }
  //   return left;
  // }

  // public int maxTwoEvents(int[][] events) {
  //   Arrays.sort(events, new Comparator<int[]>() {
  //     @Override
  //     public int compare(int[] event1, int[] event2) {
  //       return event1[0] - event2[0];
  //     }
  //   });
  //   int n = events.length;
  //   int maxSum = 0;
  //   for (int idx = 0; idx < n; idx++) {
  //     int[] event = events[idx];
  //     if (event[2] > maxSum) maxSum = event[2];
      
  //     int next = binarySearch(events, event[1] + 1);
  //     if (next < n) {
  //       int maxNext = events[next++][2];
  //       while (next < n){
  //         if (events[next][2] > maxNext) maxNext = events[next][2];
  //         next++;
  //       }
  //       if (event[2] + maxNext > maxSum) maxSum = event[2] + maxNext;
  //     }
  //   }
  //   return maxSum;
  // }
}
