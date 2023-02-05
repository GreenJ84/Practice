# A company has n employees with a unique ID for each employee from 0 to n - 1. The head of the company is the one with headID.
# Each employee has one direct manager given in the manager array where manager[i] is the direct manager of the i-th employee, manager[headID] = -1. Also, it is guaranteed that the subordination relationships have a tree structure.
# The head of the company wants to inform all the company employees of an urgent piece of news. He will inform his direct subordinates, and they will inform their subordinates, and so on until all employees know about the urgent news.
# The i-th employee needs informTime[i] minutes to inform all of his direct subordinates (i.e., After informTime[i] minutes, all his direct subordinates can start spreading the news).
# Return the number of minutes needed to inform all the employees about the urgent news.

from typing import List
from collections import defaultdict

# Horrible memory usage (7%)
class Solution:
    def numOfMinutes(self, n: int, headID: int, manager: List[int], informTime: List[int]) -> int:
        ans = 0
        dict = defaultdict(list)
        visited = set()

        for i in range(n):
            if manager[i] != -1:
                dict[manager[i]].append(i)

        def callSubs(node):
            if node in visited: return 0
            if not node in dict: return 0
            visited.add(node)
            curr = 0
            for x in dict[node]:
                curr = max(curr, callSubs(x))
            return curr + informTime[node]
        return callSubs(headID)

s = Solution()
# print(s.numOfMinutes(1, 0, [-1], [0]))
# print(s.numOfMinutes(6, 2, [2,2,-1,2,2,2], [0,0,1,0,0,0]))
# print(s.numOfMinutes(7, 6, [1,2,3,4,5,6,-1], [0,6,5,4,3,2,1]))
print(s.numOfMinutes(15, 0, [-1,0,0,1,1,2,2,3,3,4,4,5,5,6,6], [1,1,1,1,1,1,1,0,0,0,0,0,0,0,0]))
