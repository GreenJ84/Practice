// There is an undirected star graph consisting of n nodes labeled from 1 to n. A star graph is a graph where there is one center node and exactly n - 1 edges that connect the center node with every other node.

// You are given a 2D integer array edges where each edges[i] = [ui, vi] indicates that there is an edge between the nodes ui and vi. Return the center of the given star graph.

public class FindCenterOfStarGraph{
  public static void main(String[] args) {
    Solution solution = new Solution();

    testFindCenter(1, new int[][]{{1,2},{2,3},{4,2}}, 2, solution);
    testFindCenter(2, new int[][]{{1,2},{5,1},{1,3},{1,4}}, 1, solution);
  }

  private static void testFindCenter(int testNum, int[][] edges, int expected, Solution s){
    int result = s.findCenter(edges);

    System.out.println(String.format(
      "Test %d: %d / %d (%b)",
      testNum,
      result,
      expected,
      result == expected
    ));
  }
}

class Solution {
  public int findCenter(int[][] edges) {
      int[] nodes = new int[edges.length + 1];
      for (int[] edge : edges) {
        nodes[edge[0] - 1]++;
        nodes[edge[1] - 1]++;
      }
      for (int i = 0; i < nodes.length; i++) {
        if (nodes[i] == edges.length) {
          return i + 1;
        }
      }
      return -1;
  }
}