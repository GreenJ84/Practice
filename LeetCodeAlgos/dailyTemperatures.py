# Given an array of integers temperatures represents the daily temperatures, return an array answer such that answer[i] is the number of days you have to wait after the ith day to get a warmer temperature. If there is no future day for which this is possible, keep answer[i] == 0 instead.

from typing import List

class Solution:
    def dailyTemperatures(self, temperatures: List[int]) -> List[int]:

        result = [0] * len(temperatures)

        if min(temperatures) == max(temperatures): return result

        stack = []

        for index, temp in enumerate(temperatures):
            while stack and temperatures[stack[-1]] < temp:
                prev_temp = stack.pop()
                result[prev_temp] = index - prev_temp
            stack.append(index)

        return result


# class Solution:
#     def dailyTemperatures(self, temperatures: List[int]) -> List[int]:
#         if min(temperatures) == max(temperatures):
#             return [0]*len(temperatures)

#         for i in range(len(temperatures)):
#             j = i
#             while j<len(temperatures):
#                 if temperatures[j]>temperatures[i]:
#                     temperatures[i]=j-i
#                     break
#                 j+=1
#             if j == len(temperatures):
#                 temperatures[i]=0
#         return temperatures


s = Solution()
print(s.dailyTemperatures([73,74,75,71,69,72,76,73]))
print(s.dailyTemperatures([30,40,50,60]))
print(s.dailyTemperatures([30,60,90]))
print(s.dailyTemperatures([30,30,30,30,30,30,30,30,30,30,30,30,30]))
