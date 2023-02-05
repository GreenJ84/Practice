# You are given the head of a singly linked-list. The list can be represented as:
    # L0 → L1 → … → Ln - 1 → Ln
# Reorder the list to be on the following form:
    # L0 → Ln → L1 → Ln - 1 → L2 → Ln - 2 → …
# You may not modify the values in the list's nodes. Only nodes themselves may be changed.

# Definition for singly-linked list.
from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        # Set two list runners
        runner, runner2 = head, head.next
        # If only one Node return that node
        if not runner2 or not runner2.next:
            return head
        # While the fast runner still in list run down list
        while runner2:
            runner2 = runner2.next
            if not runner2: break
            runner2 = runner2.next
            # slow runner is looking for the halfway point
            runner = runner.next

        # Move runner 2 to start of second half
        runner2 = runner.next
        # Cut the first half off (still has the head)
        runner.next = None
        # Move slow runner onto the second half
        runner = runner2.next
        # Remove second half's "head"
        runner2.next = None
        # Run throught the list reversing pointers 
        while runner and runner.next:
            temp = runner.next
            runner.next = runner2
            runner2 = runner
            runner = temp
        if runner:
            runner.next = runner2
        else: 
            runner = runner2

        # Runner corrently points to the second half reversed
        # Set Runner2 to the head of the first half (which is always gonna be even or 1 longer)
        runner2 = head
        while runner:
            # Save the next value for each section
            temp = runner2.next
            temp2 = runner.next
            # Add the first value from the second section to the first section
            runner2.next = runner
            #Add the rest of the first section back on
            runner.next = temp
            runner2 = temp
            # Reset runner on the second section
            runner = temp2
        return head

def check(head):
    runner = head
    while runner:
        print(runner.val)
        runner=runner.next

s = Solution()
print(check(s.reorderList(
    ListNode(1, ListNode(2))
)))

print(check(s.reorderList(
    ListNode(1, ListNode(2, ListNode(3)))
)))

print(check(s.reorderList(
    ListNode(1, ListNode(2, ListNode(3, ListNode(4))))
)))

print(check(s.reorderList(
    ListNode(1, ListNode(2, ListNode(3, ListNode(4, ListNode(5)))))
)))
