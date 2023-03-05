# Given a characters array tasks, representing the tasks a CPU needs to do, where each letter represents a different task. Tasks could be done in any order. Each task is done in one unit of time. For each unit of time, the CPU could complete either one task or just be idle.
# However, there is a non-negative integer n that represents the cooldown period between two same tasks (the same letter in the array), that is that there must be at least n units of time between any two same tasks.
# Return the least number of units of times that the CPU will take to finish all the given tasks.

from typing import List


class Solution:
    def leastInterval(self, tasks: List[str], n: int) -> int:
        if n == 0: return len(tasks)
        hold = {}
        for task in tasks:
            if task in hold:
                hold[task] += 1
            else:
                hold[task] = 1
        hold = [item for key, item in list(sorted(hold.items(), key= lambda x: x[1]))]
        
        runTime = 0
        while max(hold) > 0:
            for i in range(1, n+2):
                while i <= len(hold) and hold[-i] == 0:
                    hold.pop(-i)
                    if i > len(hold):
                        break
                if i <= len(hold):
                    hold[-i] -= 1
                runTime += 1
                if max(hold) == 0: 
                    break
        return runTime
    
s = Solution()
# print(s.leastInterval(["A","A","A","B","B","B"], 2))
# print(s.leastInterval(["A","A","A","B","B","B"], 0))
# print(s.leastInterval(["A","A","A","A","A","A","B","C","D","E","F","G"], 2))
print(s.leastInterval(["A","A","A","B","B","B", "C","C","C", "D", "D", "E"]
, 2))