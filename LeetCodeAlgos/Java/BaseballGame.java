// You are keeping the scores for a baseball game with strange rules. At the beginning of the game, you start with an empty record.
// You are given a list of strings operations, where operations[i] is the ith operation you must apply to the record and is one of the following:
  // An integer x. -- Record a new score of x.
  // '+'. -- Record a new score that is the sum of the previous two scores.
  // 'D'. -- Record a new score that is the double of the previous score.
  // 'C'. -- Invalidate the previous score, removing it from the record.
// Return the sum of all the scores on the record after applying all the operations.
// The test cases are generated such that the answer and all intermediate calculations fit in a 32-bit integer and that all operations are valid.
import java.util.*;

class BaseballGame {
  public static void main(String[] args) {
    BaseballGame s = new BaseballGame();
    
    String[] test1 = new String[] {"5", "2", "C", "D", "+"};
    int ans1 = s.calPoints(test1);
    System.out.println(ans1);
    assert ans1 == 30;

    String[] test2 = new String[] {"5","-2","4","C","D","9","+","+"};
    int ans2 = s.calPoints(test2);
    System.out.println(ans2);
    assert ans2 == 27;
  }

  public int calPoints(String[] operations) {
    int sum = 0;
    ArrayList<Integer> scores = new ArrayList<Integer>();
    for (int i = 0; i < operations.length; i++) {
      String op = operations[i];
      Integer prev = null;
      int prevSpot = scores.size() - 1;
      switch (op) {
        case "+":
          prev = scores.get(prevSpot);
          Integer befPrev = scores.get(prevSpot - 1);
          scores.add(prev + befPrev);
          sum += prev + befPrev;
          break;
        case "D":
          prev = scores.get(prevSpot);
          scores.add(prev * 2);
          sum += prev * 2;
          break;
        case "C":
          sum -= scores.get(prevSpot);
          scores.remove(prevSpot);
          break;
        default:
          Integer score = Integer.parseInt(op);
          scores.add(score);
          sum += score;
          break;
      }
    }
    return sum;
  }


  //     public int calPoints(String[] operations) {
  //         ArrayList<Integer> scores = new ArrayList<Integer>();
  //         for (int i = 0; i < operations.length; i++) {
  //           String op = operations[i];
  //           Integer prev = null;
  //           switch (op) {
  //             case "+":
  //               prev = scores.get(scores.size() - 1);
  //               Integer befPrev = scores.get(scores.size() - 2);
  //               scores.add(prev + befPrev);
  //               break;
  //             case "D":
  //               prev = scores.get(scores.size() - 1);
  //               scores.add(prev * 2);
  //               break;
  //             case "C":
  //               scores.remove(scores.size() - 1);
  //               break;
  //             default:
  //               scores.add(Integer.parseInt(op));
  //               break;
  //           }
  //         }
  //         int sum = 0;
  //         for (int score: scores) {
  //           sum += score;
  //         }
  //         return sum;
  //     }
}