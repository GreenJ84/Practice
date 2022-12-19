# Given the head of a linked list, return the list after sorting it in ascending order.

from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
        

class Solution:
    def sortList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if not head or not head.next: return head

        vals = []
        while head:
            vals.append(head.val)
            head=head.next
        vals.sort()

        return ListNode(','.join(map(str, vals)))
# class Solution:
#     def sortList(self, head: Optional[ListNode]) -> Optional[ListNode]:
#         if not head or not head.next: return head

#         vals = []
#         while head:
#             vals.append(head.val)
#             head=head.next
#         vals.sort()
#         setup = runner = ListNode()

#         for i in vals:
#             runner.next = ListNode(i)
#             runner = runner.next
#         return setup.next

