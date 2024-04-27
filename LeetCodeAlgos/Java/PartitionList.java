public class PartitionList {
  public static void main(String[] args) {}
  static void testPartitionList(int testNum, ListNode head, int x, ListNode expected, Solution s) {
    ListNode result = s.partition(head, x);

    System.out.println(String.format(
      "Test %d: %s /x %s (%b)",
      testNum,
      result,
      expected,
      result.equals(expected)
    ));
  }
}

class ListNode {
    int val;
    ListNode next;
    ListNode() {}
    ListNode(int val) { this.val = val; }
    ListNode(int val, ListNode next) { this.val = val; this.next = next; }
}

class Solution {
    public ListNode partition(ListNode head, int x) {
        ListNode gte = new ListNode(0);
        ListNode mutator = gte;
        ListNode lt = new ListNode(0);
        ListNode runner = lt;
        while (head != null) {
          if (head.val >= x) {
            mutator.next = head;
            mutator = mutator.next;
          } else {
            runner.next = head;
            runner = runner.next;
          }
          head = head.next;
        }
        mutator.next = null;
        runner.next = gte.next;
        return lt.next;
    }
}