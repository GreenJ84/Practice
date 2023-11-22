# You are given an array of unique integers salary where salary[i] is the salary of the ith employee.
# Return the average salary of employees excluding the minimum and maximum salary. Answers within 10-5 of the actual answer will be accepted.

from functools import reduce
from typing import List


class Solution:
    def average(self, salary: List[int]) -> float:
        x = reduce(lambda x, y: x+y, salary)
        return (x-max(salary)-min(salary))/(len(salary)-2)

s = Solution()
print(s.average([4000,3000,1000,2000]))
print(s.average([3000,1000,2000]))