// The school cafeteria offers circular and square sandwiches at lunch break, referred to by numbers 0 and 1 respectively. All students stand in a queue. Each student either prefers square or circular sandwiches.

// The number of sandwiches in the cafeteria is equal to the number of students. The sandwiches are placed in a stack. At each step:
  // If the student at the front of the queue prefers the sandwich on the top of the stack, they will take it and leave the queue.
  // Otherwise, they will leave it and go to the queue's end.
// This continues until none of the queue students want to take the top sandwich and are thus unable to eat.

// You are given two integer arrays students and sandwiches where sandwiches[i] is the type of the i​​​​​​th sandwich in the stack (i = 0 is the top of the stack) and students[j] is the preference of the j​​​​​​th student in the initial queue (j = 0 is the front of the queue). Return the number of students that are unable to eat.
import java.util.*;
import java.util.stream.Collectors;

class Solution {
  public int countStudents(int[] students, int[] sandwiches) {
    List<Integer> list = Arrays.stream(students).boxed().collect(Collectors.toList());
    // Idx for Top student in queue
    int studentTop = 0;
    // Iterate all sandwiches
    for (int i = 0; i < sandwiches.length; i++) {
      boolean studentFed = false;
      // Look at all students in the queue
      for (int j = 0; j < list.size(); j++) {
        // Start from last top student and track around array
        int queueIdx = (studentTop + j) % list.size();
        // If a student approves of sandwich
        if (list.get(queueIdx) == sandwiches[i]) {
          // Move Queue top to current student
          studentTop = queueIdx;
          // Remove student for the following
          list.remove(studentTop);
          studentFed = true;
          break;
        }
      }
      // No students wanted top sandwich
      if (!studentFed) {
        return list.size();
      }
    }
    // all students fed
    return 0;
  }
}


public class NumberOfStudentsUnableToEatLunch {
  public static void main(String[] args) {
    Solution s = new Solution();
    

    NumberOfStudentsUnableToEatLunch.testCountStudents(1, new int[]{1,1,0,0}, new int[]{0,1,0,1}, 0, s);
    NumberOfStudentsUnableToEatLunch.testCountStudents(1, new int[]{1,1,1,0,0,1}, new int[]{1,0,0,0,1,1}, 3, s);
  }

  public static void testCountStudents(int testNum, int[] students, int[] sandwiches, int expected, Solution s) {
    int result = s.countStudents(students, sandwiches);
    
    System.out.println(
      String.format(
        "Test #%d:\nstudents: %s\nsandwiches: %s\nexpected: %d\nresult: %d",
        testNum, Arrays.toString(students), Arrays.toString(sandwiches), expected, result
      )
    );
  };
}