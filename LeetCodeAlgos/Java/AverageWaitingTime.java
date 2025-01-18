public class AverageWaitingTime {
  public static void main(String[] args) {
    AverageWaitingTime obj = new AverageWaitingTime();

    testAverageWaitingTime(1, new int[][]{{1, 2}, {2, 5}, {4, 3}}, 5.0, obj);

    testAverageWaitingTime(2, new int[][]{{5, 2}, {5, 4}, {10, 3}, {20, 1}}, 3.25000, obj);

  }

  private static void testAverageWaitingTime(int testNum, int[][] customers, double expected, AverageWaitingTime s){
    double result = s.averageWaitingTime(customers);

    System.out.println(String.format(
      "Test %d: %.2f / %.2f (%b)",
      testNum,
      result,
      expected,
      Math.abs(result - expected) < 0.0001
    ));
  }

  public double averageWaitingTime(int[][] customers) {
    double res = 0.0;
    int curTime = customers[0][0];

    for (int[] customer : customers) {
      if (customer[0] > curTime) {
        curTime = customer[0];
      }
      curTime += customer[1];
      res += curTime - customer[0];
    }
    return res / customers.length;
  }

  //! Slower runtime on a traditional for loop?
  //   public double averageWaitingTime(int[][] customers) {
  //     double res = 0.0;
  //     int curTime = customers[0][0];

  //     for (int idx = 0; idx < customers.length; idx++) {
  //       if (customers[idx][0] > curTime) {
  //         curTime = customers[idx][0];
  //       }
  //       curTime += customers[idx][1];
  //       res += curTime - customers[idx][0];
  //     }
  //     return res / customers.length;
  //   }
}
