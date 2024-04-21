// There is a bi-directional graph with n vertices, where each vertex is labeled from 0 to n - 1 (inclusive). The edges in the graph are represented as a 2D integer array edges, where each edges[i] = [ui, vi] denotes a bi-directional edge between vertex ui and vertex vi. Every vertex pair is connected by at most one edge, and no vertex has an edge to itself.

// You want to determine if there is a valid path that exists from vertex source to vertex destination.

// Given edges and the integers n, source, and destination, return true if there is a valid path from source to destination, or false otherwise.

import java.util.*;

public class FindIfPathExistsInGraph {

}

class Solution {
  private HashSet<Integer> visited = new HashSet<>();
  private HashMap<Integer, ArrayList<Integer>> adjList = new HashMap<>();
  public boolean validPath(int n, int[][] edges, int source, int destination) {
    if (source == destination) { return true;}
    for (int[] edge : edges) {
      adjList.putIfAbsent(edge[0], new ArrayList<>());
      adjList.putIfAbsent(edge[1], new ArrayList<>());
      adjList.get(edge[0]).add(edge[1]);
      adjList.get(edge[1]).add(edge[0]);
    }
    return hasPath(n, edges, source, destination);
  }
  public boolean hasPath(int n, int[][] edges, int source, int destination) {
    visited.add(source);
    if (adjList.containsKey(source)){
      for (int dest : adjList.get(source)) {
        if (dest == destination) {
          return true;
        }
        else if (!visited.contains(dest)) {
          if (hasPath(n, edges, dest, destination)) {
            return true;
          }
        }
      }
    }
    return false;
  }
}