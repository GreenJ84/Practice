# You are given an array people where people[i] is the weight of the ith person, and an infinite number of boats where each boat can carry a maximum weight of limit. Each boat carries at most two people at the same time, provided the sum of the weight of those people is at most limit.
# Return the minimum number of boats to carry every given person.

from typing import List


class Solution:
    def numRescueBoats(self, people: List[int], limit: int) -> int:
        people.sort()
        boats = 0
        while people and people[-1] == limit:
            boats += 1
            people.pop()

        while people:
            start = people[-1]
            window = 0
            people.pop()
            for i in range(len(people)):
                if start + window + people[i] > limit:
                    people = people[i:]
                    break
                else:
                    window += people[i]
            if window == sum(people):
                people = []
            boats+= 1
        return boats


s = Solution()
print(s.numRescueBoats([1,2], 3)) # 1
print(s.numRescueBoats([3,2,2,1], 3)) # 3
print(s.numRescueBoats([3,5,3,4], 5)) # 4
print(s.numRescueBoats([2, 2], 5)) # 1
print(s.numRescueBoats([2,2,2,2,2], 2)) # 5
print(s.numRescueBoats([1, 2, 3], 7)) # 1
print(s.numRescueBoats([2, 3, 5], 5)) # 2