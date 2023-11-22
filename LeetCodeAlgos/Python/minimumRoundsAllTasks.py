# You are given a 0-indexed integer array tasks, where tasks[i] represents the difficulty level of a task. In each round, you can complete either 2 or 3 tasks of the same difficulty level.
# Return the minimum rounds required to complete all the tasks, or -1 if it is not possible to complete all the tasks.

from typing import List


#* 82% Runtime/ 96% Memory
class Solution:
    def minimumRounds(self, tasks: List[int]) -> int:
        cnt = {}
        for i in tasks:
            if i not in cnt:
                cnt[i] = 1
            else:
                cnt[i]+=1
        
        reps=0
        for j in cnt.values():
            if j == 1:
                return -1
            elif j%3==0:
                reps+=j//3
                continue
            elif j%3==1 or j%3==2:
                reps+=j//3+1
        return reps

s = Solution()
print(s.minimumRounds([2,2,3,3,2,4,4,4,4,4]))
print(s.minimumRounds([2,3,3]))
print(s.minimumRounds([1]))