# Given two integer arrays pushed and popped each with distinct values, return true if this could have been the result of a sequence of push and pop operations on an initially empty stack, or false otherwise.

from typing import List


class Solution:
    def validateStackSequences(self, pushed: List[int], popped: List[int]) -> bool:
        ans = []
        for i in pushed:
            ans.append(i)
            while ans and ans[-1] == popped[0]:
                ans.pop()
                popped = popped[1:]
        return True if len(ans) == 0 else False

s = Solution()
print(s.validateStackSequences([1,2,3,4,5], [4,5,3,2,1]))
print(s.validateStackSequences([1,2,3,4,5], [4,3,5,1,2]))