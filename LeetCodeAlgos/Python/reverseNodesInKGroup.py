# Given the head of a linked list, reverse the nodes of the list k at a time, and return the modified list.
# k is a positive integer and is less than or equal to the length of the linked list. If the number of nodes is not a multiple of k then left-out nodes, in the end, should remain as it is.
# You may not alter the values in the list's nodes, only nodes themselves may be changed.


from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def reverseKGroup(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        if not head or not head.next: return head
        listHold = ListNode(0, head)

        runner = head
        frontTemp = listHold
        while runner:
            sec = []
            for i in range(k):
                sec.append(runner)
                if i != k-1:
                    runner = runner.next
                    if not runner:
                        return listHold.next
            backTemp = runner.next
            for i in range(len(sec)-1, 0, -1):
                sec[i].next = sec[i-1]
            frontTemp.next = sec[len(sec)-1]
            sec[0].next = backTemp
            frontTemp = sec[0]
            runner = backTemp
        return listHold.next

s = Solution()
print(s.reverseKGroup(
    ListNode(1, ListNode(2, ListNode(3, ListNode(4, ListNode(5))))),
    2
))
