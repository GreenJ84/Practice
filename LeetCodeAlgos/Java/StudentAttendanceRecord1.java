// You are given a string s representing an attendance record for a student where each character signifies whether the student was absent, late, or present on that day. The record only contains the following three characters:

// 'A': Absent.
// 'L': Late.
// 'P': Present.
// The student is eligible for an attendance award if they meet both of the following criteria:

// The student was absent ('A') for strictly fewer than 2 days total.
// The student was never late ('L') for 3 or more consecutive days.
// Return true if the student is eligible for an attendance award, or false otherwise.

class Solution {
    public boolean checkRecord(String s) {
        int lateCount = 0;
        int absentCount = 0;
        for (char c : s.toCharArray()) {
          if (c == 'A') {
            absentCount++;
            if (absentCount >= 2) {
              return false;
            }
            lateCount = 0;
          } else if (c == 'L') {
            lateCount++;
            if (lateCount >= 3) {
              return false;
            }
          }
          else {
            lateCount = 0;
          }
        }
      return true;
    }
}

class StudentAttendanceRecord1 {
  public static void main(String[] args) {
    Solution s = new Solution();
    System.out.println(s.checkRecord("PPALLP"));
    System.out.println(s.checkRecord("PPALLL"));
  }
}