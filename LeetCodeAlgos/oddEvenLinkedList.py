# Given the head of a singly linked list, group all the nodes with odd indices together followed by the nodes with even indices, and return the reordered list.
# The first node is considered odd, and the second node is even, and so on.
# Note that the relative order inside both the even and odd groups should remain as it was in the input.
#! You must solve the problem in O(1) extra space complexity and O(n) time complexity.

from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def oddEvenList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        print(head)
        if not head or not head.next or not head.next.next:
            return head

        evenList, runOdd, step = ListNode(), head, 1

        evenList.next = runOdd.next
        runEven = evenList.next
        runOdd.next = runOdd.next.next
        runOdd = runOdd.next
        runEven.next = None

        while runOdd:
            if runOdd.next:
                runEven.next = runOdd.next
                runEven = runEven.next
                if runOdd.next.next:
                    runOdd.next = runOdd.next.next
                    runOdd = runOdd.next
                    runEven.next = None
                    continue
            runOdd.next = evenList.next
            break
        return head
        
# class Solution:
#     def oddEvenList(self, head: Optional[ListNode]) -> Optional[ListNode]:
#         if not head or not head.next or not head.next.next:
#             return head

#         evenList, runOdd = head.next, head

#         runOdd.next = runOdd.next.next
#         runOdd = runOdd.next
#         if runOdd.next: 
#             evenList.next = runOdd.next
#             runEven = evenList.next
#             runOdd.next = runOdd.next.next

#         while runOdd:
#             if runOdd.next:
#                 runEven.next = runOdd.next
#                 runEven = runEven.next
#                 if runOdd.next.next:
#                     runOdd.next = runOdd.next.next
#                     runOdd = runOdd.next
#                     runEven.next = None
#                     continue
#             break
#         runOdd.next = evenList.next
#         return head

s = Solution()

# print(s.oddEvenList(ListNode()))
# print(s.oddEvenList(ListNode( 1 ) ))
# print(s.oddEvenList(ListNode( 1, ListNode( 2 )) ))
print(s.oddEvenList(ListNode( 1, ListNode( 2, ListNode( 3, ListNode( 4, ListNode( 5, ListNode(6 )))))) ))
print(s.oddEvenList(ListNode( 2, ListNode( 1, ListNode( 3, ListNode( 5, ListNode( 6, ListNode( 4, ListNode( 7 ))))))) ))