// You have planned some train traveling one year in advance. The days of the year in which you will travel are given as an integer array days. Each day is an integer from 1 to 365.

// Train tickets are sold in three different ways:

// a 1-day pass is sold for costs[0] dollars,
// a 7-day pass is sold for costs[1] dollars, and
// a 30-day pass is sold for costs[2] dollars.
// The passes allow that many days of consecutive travel.

// For example, if we get a 7-day pass on day 2, then we can travel for 7 days: 2, 3, 4, 5, 6, 7, and 8.
// Return the minimum number of dollars you need to travel every day in the given list of days.



public class MinimumCostForTickets {
  public static void main(String[] args) {
    MinimumCostForTickets obj = new MinimumCostForTickets();

    System.out.println(obj.mincostTickets(new int[]{1,4,6,7,8,20}, new int[]{2,7,15}));
    System.out.println(obj.mincostTickets(new int[]{1,2,3,4,5,6,7,8,9,10,30,31}, new int[]{2,7,15}));
  }

  public int mincostTickets(int[] days, int[] costs) {
    int[] day_cost = new int[366];
    day_cost[0] = 0;

    int day = 0;
    for (int i = 1; i < 366; i++) {
      if (i != days[day]) {
        day_cost[i] = day_cost[i - 1];
        continue;
      }
      day_cost[i] = day_cost[i - 1] + costs[0];
      day_cost[i] = Math.min(day_cost[i], day_cost[Math.max(0, i - 7)] + costs[1]);
      day_cost[i] = Math.min(day_cost[i], day_cost[Math.max(0, i - 30)] + costs[2]);
      day++;
      if (day >= days.length) return day_cost[i];
    }
    return day_cost[365];
  }
}
