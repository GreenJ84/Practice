// You are given a map of a server center, represented as a m * n integer matrix grid, where 1 means that on that cell there is a server and 0 means that it is no server. Two servers are said to communicate if they are on the same row or on the same column.

// Return the number of servers that communicate with any other server.

import java.util.Arrays;

public class CountServersThatCommunicate {
  public static void main(String[] args) {
    CountServersThatCommunicate obj = new CountServersThatCommunicate();

    int[][] grid = {{1,0}, {0,1}};
    System.out.println(obj.countServers(grid));

    int[][] grid2 = {{1,0}, {1,1}};
    System.out.println(obj.countServers(grid2));

    int[][] grid3 = {{1,1,0,0}, {0,0,1,0}, {0,0,1,0}, {0,0,0,1}};
    System.out.println(obj.countServers(grid3));
  }

  public int countServers(int[][] grid) {
    int rows = grid.length;
    int cols = grid[0].length;
    int ans = 0;

    int[] validRow = new int[rows];
    Arrays.fill(validRow, 0);
    int[] validCol = new int[cols];
    Arrays.fill(validCol, 0);
    for (int row = 0; row < rows; row++){
      for (int col = 0; col < cols; col++){
        if (grid[row][col] == 1) {
          validRow[row]++;
          validCol[col]++;
        }
      }
    }

    for (int row = 0; row < rows; row++){
      for (int col = 0; col < cols; col++){
        if (grid[row][col] == 1 && (
          validRow[row] > 1 || validCol[col] > 1
        )) {
          ans += 1;
        }
      }
    }
    return ans;
  }

  //! Error: Does not accurately track valid servers.
  // public int countServers(int[][] grid) {
  //   int rows = grid.length;
  //   int cols = grid[0].length;
  //   int ans = 0;

  //   // Key existence equates a found server in the column.
  //   // Value truthiness equates valid counting of initial column server
  //   HashMap<Integer, Boolean> connectedCols = new HashMap<Integer, Boolean>(cols);

  //   for (int row = 0; row < rows; row++){
  //     // Idx 1 truthiness equates a found server in row.
  //     // Idx 2 truthiness equates valid accounting of initial row server
  //     boolean[] validRow = new boolean[] {false, false};
  //     for (int col = 0; col < cols; col++){
  //       if (grid[row][col] == 1) {
  //         boolean accountedServer = false;
  //         if (validRow[0]){
  //           accountedServer = true;
  //           // 2 added to account for a new found server w/ establishing the initial server
  //           if (!validRow[1]){
  //             ans += 2;
  //             validRow[1] = true;
  //           }
  //           // 1 for every server after the initial server is accounted for
  //           else {
  //             ans += 1;
  //           }
  //         } else {
  //           validRow[0] = true;
  //         }

  //         if (connectedCols.containsKey(col)){
  //           boolean accountedColumnServer = connectedCols.get(col);
  //           if (!accountedColumnServer && !accountedServer){
  //             ans += 2;
  //             connectedCols.put(col, true);
  //             validRow[1] = true;
  //           }
  //           else if (!accountedColumnServer){
  //             ans += accountedServer ? 1 : 2;
  //           }
  //           else {
  //             ans += 1;
  //           }
  //         } else {
  //           connectedCols.put(col, accountedServer);
  //         }
  //       }
  //     }
  //   }
  //   return ans;
  // }
}
