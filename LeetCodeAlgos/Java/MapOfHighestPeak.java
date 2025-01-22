import java.util.*;

public class MapOfHighestPeak {
  public static void main(String[] args) {
    MapOfHighestPeak obj = new MapOfHighestPeak();

    int[][] isWater = {{0,1},{0,0}};
    System.out.println(Arrays.deepToString(obj.highestPeak(isWater)));

    int[][] isWater2 = {{0,0,1},{1,0,0},{0,0,0}};
    System.out.println(Arrays.deepToString(obj.highestPeak(isWater2)));
  }
  public int[][] highestPeak(int[][] isWater) {
    int[][] dirs = { {0, 1}, {0, -1}, {1, 0}, {-1, 0} };
    int rows = isWater.length;
    int columns = isWater[0].length;

    int[][] heightMap = new int[rows][columns];
    for (int[] row : heightMap) {
        Arrays.fill(row, -1);
    }

    Queue<int[]> queue = new LinkedList<>();
    for (int row = 0; row < rows; row++) {
        for (int col = 0; col < columns; col++) {
            if (isWater[row][col] == 1) {
                queue.add(new int[] { row, col });
                heightMap[row][col] = 0;
            }
        }
    }

    int heightOfNextLayer = 1;
    while (!queue.isEmpty()) {
        int layerSize = queue.size();
        for (int i = 0; i < layerSize; i++) {
            int[] currentCell = queue.poll();

            for (int[] dir : dirs) {
                int neighborX = currentCell[0] + dir[0];
                int neighborY = currentCell[1] + dir[1];
                if (
                    withinMap(neighborX, neighborY, rows, columns) &&
                    heightMap[neighborX][neighborY] == -1
                ) {
                    heightMap[neighborX][neighborY] = heightOfNextLayer;
                    queue.add(new int[] { neighborX, neighborY });
                }
            }
        }
        heightOfNextLayer++;
    }

      return heightMap;
  }

  private boolean withinMap(int x, int y, int rows, int columns) {
      return x >= 0 && y >= 0 && x < rows && y < columns;
  }
}