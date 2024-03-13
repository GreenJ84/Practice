// Given the head of a linked list, we repeatedly delete consecutive sequences of nodes that sum to 0 until there are no such sequences.

// After doing so, return the head of the final linked list.  You may return any such answer.

 

// (Note that in the examples below, all sequences are serializations of ListNode objects.)

public class ListNode {
  public int val;
  public ListNode next;
  public ListNode(int val=0, ListNode next=null) {
      this.val = val;
      this.next = next;
  }
  public override string ToString(){
    StringBuilder listRep = new StringBuilder();
    listRep.Append("[");
    ListNode list = this;
    while (list != null){
      listRep.Append($"{list.val},");
      list = list.next;
    }
    listRep.Remove(listRep.Length - 1, 1);
    listRep.Append("]");
    return listRep.ToString();
  }
}

class Solution {
  public ListNode RemoveZeroSumSublists(ListNode head) {
    ListNode temp = new ListNode(0, head);
    Dictionary<int, ListNode> prefixSumToNode = new Dictionary<int, ListNode>();
    int prefixSum = 0;

    for (ListNode current = temp; current != null; current = current.next) {
      prefixSum += current.val;
      if (prefixSumToNode.ContainsKey(prefixSum)) {
        ListNode prev = prefixSumToNode[prefixSum];
        ListNode toRemove = prev.next;
        int p = prefixSum + (toRemove != null ? toRemove.val : 0);
        while (p != prefixSum) {
          prefixSumToNode.Remove(p);
          toRemove = toRemove.next;
          p += (toRemove != null ? toRemove.val : 0);
        }
        prev.next = current.next;
      } else {
        prefixSumToNode[prefixSum] = current;
      }
    }
    return temp.next;
  }
}

// public class Solution {
//   public ListNode RemoveZeroSumSublists(ListNode head) {
//     // Holder for tip of LL
//     ListNode temp = new ListNode(-69, head);
//     // Positioner for mutating List
//     ListNode mutator = temp;
//     // Runs down the list for checking updates
//     ListNode runner = head;
//     // Keep track of the last node seen
//     ListNode lastSeen = null;
//     // Maintain a running window sum
//     int runningSum = head.val;

//     while (runner.next != null) {
//       lastSeen = runner; 
//       runner = runner.next; 
//       runningSum += runner.val; 
//       // Whole window Zero sumation
//       if (runningSum == 0){
//         // Mutator still at starting position? Mutate head
//         runner = RemoveWindow(temp, mutator, runner);
//       } 
//       // Direct node to node cancellation
//       else if (lastSeen.val == runner.val * -1){
//         if (mutator == lastSeen){
//           mutator = temp;
//         }
//         while (mutator.next != lastSeen){
//           mutator = mutator.next;
//         }
//         runner = RemoveWindow(temp, mutator, runner);
//       }
//     }

//     // Oiginal Linked List should be modified with removas
//     return temp.next;
//   }

//   public ListNode RemoveWindow(ListNode temp, ListNode mutator, ListNode runner) {
//     bool headOFList = temp == mutator;
//     // Mutate List with a window splice
//     mutator.next = runner.next;
//     if (headOFList){
//       temp.next = runner.next;
//     }
//     runner.next = null;
//     return mutator;
//   }
// }

public static void TestRemoveZeroSumSublists(ListNode input, ListNode expected, int testNumber) {
  Solution solution = new Solution();

  string inputDisplay = input.ToString();
  string expectDisplay = expected.ToString();
  string result = solution.RemoveZeroSumSublists(input).ToString();
  bool passed = result == expectDisplay;

  Console.WriteLine($"Test {testNumber}: {inputDisplay} => {result}/{expectDisplay}({passed})");
}

TestRemoveZeroSumSublists( 
  new ListNode(1, new ListNode(2, new ListNode(-3, new ListNode(3, new ListNode(1))))), 
  new ListNode(3, new ListNode(1)),
  1
);
TestRemoveZeroSumSublists(
  new ListNode(1, new ListNode(2, new ListNode(3, new ListNode(-3, new ListNode(4))))), 
  new ListNode(1, new ListNode(2, new ListNode(4))),
  2
);
TestRemoveZeroSumSublists(
  new ListNode(1, new ListNode(2, new ListNode(3, new ListNode(-3, new ListNode(-2))))), 
  new ListNode(1),
  3
);