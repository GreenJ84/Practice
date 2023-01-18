# You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
# You may assume the two numbers do not contain any leading zero, except the number 0 itself.

from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
        
class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        # Returns if either or both args are missing
        if not l1:
            return l2
        if not l2:
            return l1
        # set number holders for easier val access
        ll1, ll2 = [], []
        # Run through each LinkedList to grab vals
        runner = l1
        while runner:
            ll1.append(runner.val)
            runner = runner.next
        runner = l2
        while runner:
            ll2.append(runner.val)
            runner = runner.next
        print(ll1, ll2)
        # Set longest list and math intermediate
        work = ll1 if len(ll1)>=len(ll2) else ll2
        temp = 0
        # Do addition back to front of arrays
        for i in range(0, len(work)):
            # Conditional depending on each lists length
            if i >= len(ll1):
                temp = ll2[i]+temp
            elif i >= len(ll2):
                temp = ll1[i]+temp
            else:
                temp = ll1[i]+ll2[i]+temp
            # Once a sum is acheived add the ones digit to current spot
            work[i] = temp % 10
            # Floor Divide by ten to reove ones place
            temp = temp//10
        # If any leftover sum process it
        if temp:
            # If large then 10 process down and add to end
            while temp > 9:
                work = [temp % 10]+work[:]
                temp = temp // 10
            # add final sum digit to end
            work = work[:]+[temp]
        # Put your result into the linked list
        runner = l1
        for i in work:
            runner.next = ListNode(i)
            runner=runner.next
        return l1.next





s = Solution()
print(s.addTwoNumbers(
        ListNode(2, ListNode(4, ListNode(3))), 
        ListNode(5, ListNode(6, ListNode(4)))
        ));
print(s.addTwoNumbers( 
        ListNode(), 
        ListNode()
        ));
print(s.addTwoNumbers(
        ListNode(9, ListNode(9, ListNode(9, ListNode(9, ListNode(9, ListNode(9, ListNode(9))))))), 
        ListNode(9, ListNode(9, ListNode(9, ListNode(9))))
        ));