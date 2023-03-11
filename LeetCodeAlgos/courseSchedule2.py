# There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you must take course bi first if you want to take course ai.
    # For example, the pair [0, 1], indicates that to take course 0 you have to first take course 1.
# Return the ordering of courses you should take to finish all courses. If there are many valid answers, return any of them. If it is impossible to finish all courses, return an empty array.

from typing import List
from collections import defaultdict, deque


class Solution:
    def findOrder(self, numCourses: int, prerequisites: List[List[int]]) -> List[int]:
        hold = defaultdict(list)
        degree = defaultdict(int)
        for dest, src in prerequisites:
            hold[src].append(dest)
            degree[dest] += 1

        # our result variable
        res = []
        # a queue to hold takeable courses
        q = deque()
        # look for courses without prereqs
        for course in range(numCourses):
            if not degree[course]:
                q.append(course)

        # Run through courses
        while q:
            # take a class
            curr = q.popleft()
            res.append(curr)
            # Go through courses this was a prereq for
            for j in hold[curr]:
                # reduce the course's prereqs
                degree[j] -= 1
                # if the course no longer has prerqs, add it
                if not degree[j]:
                    q.append(j)
        return res if len(res) == numCourses else []

s = Solution()
print(s.findOrder(2, [[1,0]]))
print(s.findOrder(4, [[1,0], [2,0], [3,1], [3,2]]))
print(s.findOrder(1, []))