# Given head which is a reference node to a singly-linked list. The value of each node in the linked list is either 0 or 1. The linked list holds the binary representation of a number.
# Return the decimal value of the number in the linked list.
# The most significant bit is at the head of the linked list.

class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def getDecimalValue(self, head: ListNode) -> int:
        bin, val = [], 0
        runner = head
        while runner:
            bin.append(runner.val)
            runner = runner.next
        x=1
        for i in range(len(bin)-1, -1, -1):
            if bin[i] == 1:
                val+=x
            x*=2
        return val

s = Solution()
print(s.getDecimalValue(ListNode( 1,(ListNode( 0, ListNode( 1))))))
print(s.getDecimalValue(ListNode( 0 )))
print(s.getDecimalValue(ListNode( 0,(ListNode( 1, ListNode( 1))))))
