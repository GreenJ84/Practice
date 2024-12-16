// There is a school that has classes of students and each class will be having a final exam. You are given a 2D integer array classes, where classes[i] = [passi, totali]. You know beforehand that in the ith class, there are totali total students, but only passi number of students will pass the exam.

// You are also given an integer extraStudents. There are another extraStudents brilliant students that are guaranteed to pass the exam of any class they are assigned to. You want to assign each of the extraStudents students to a class in a way that maximizes the average pass ratio across all the classes.

// The pass ratio of a class is equal to the number of students of the class that will pass the exam divided by the total number of students of the class. The average pass ratio is the sum of pass ratios of all the classes divided by the number of the classes.

// Return the maximum possible average pass ratio after assigning the extraStudents students. Answers within 10-5 of the actual answer will be accepted.

import java.util.*;

public class MaximumAveragePassRatio {
  public static void main(String[] args) {
    MaximumAveragePassRatio obj = new MaximumAveragePassRatio();

    System.out.println(obj.maxAverageRatio(new int[][]{
      new int[]{1, 2},
      new int[]{3,5},
      new int[]{2,2}
    }, 2));
    System.out.println(obj.maxAverageRatio(new int[][]{
      new int[]{2,4},
      new int[]{3,9},
      new int[]{4,5},
      new int[]{2,10}
    }, 4));
  }

  public double maxAverageRatio(int[][] classes, int extraStudents) {
      PriorityQueue<Map.Entry<Integer, Double>> queue = new PriorityQueue<>((a, b) -> {
        return Double.compare(b.getValue(), a.getValue());
      });
      for (int idx = 0; idx < classes.length; idx++) {
        double pass = (double)classes[idx][0], total = (double)classes[idx][1];
        double potential = (pass + 1) / (total + 1) - pass / total;
        queue.offer(Map.entry(idx, potential));
      }

      for (int student = 0; student < extraStudents; student++) {
        Map.Entry<Integer, Double> entry = queue.poll();
        int idx = entry.getKey();
        classes[idx][0]++;
        classes[idx][1]++;

        double pass = (double)classes[idx][0], total = (double)classes[idx][1];
        double newPotential = (pass + 1) / (total + 1) - pass / total;

        queue.offer(Map.entry(idx, newPotential));
      }

      double average = 0.0;
      for (int[] _class : classes) {
        average += (double) _class[0] / _class[1];
      }

      average /= queue.size();

      return average;
    }
}