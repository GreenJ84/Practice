# Given an array of integers temperatures represents the daily temperatures, return an array answer such that answer[i] is the number of days you have to wait after the ith day to get a warmer temperature. If there is no future day for which this is possible, keep answer[i] == 0 instead.

from typing import List

#* Best stats so far: 79% runtime / 99.3% memory
class Solution:
    def dailyTemperatures(self, temperatures: List[int]) -> List[int]:
        if min(temperatures)==max(temperatures):
            return [0]*len(temperatures)

        stk = []
        for i in range(len(temperatures)-1):
            if temperatures[i]<temperatures[i+1]:
                temperatures[i]=1
                while stk and temperatures[stk[-1]]<temperatures[i+1]:
                    temperatures[stk[-1]] = i+1-stk[-1]
                    stk.pop()
            else:
                stk.append(i)

        if stk:
            for i in stk:
                temperatures[i]=0
        temperatures[-1]=0
        return temperatures


# class Solution:
#     def dailyTemperatures(self, temperatures: List[int]) -> List[int]:

#         result = [0] * len(temperatures)

#         if min(temperatures) == max(temperatures): return result

#         stack = []

#         for index, temp in enumerate(temperatures):
#             while stack and temperatures[stack[-1]] < temp:
#                 prev_temp = stack.pop()
#                 result[prev_temp] = index - prev_temp
#             stack.append(index)

#         return result



# class Solution:
#     def dailyTemperatures(self, temperatures: List[int]) -> List[int]:
#         if min(temperatures) == max(temperatures):
#             return [0]*len(temperatures)

#         stack = []
#         for i in range(len(temperatures)-1):
#             if temperatures[i]<temperatures[i+1]:
#                 temperatures[i] = 1
#                 while stack and temperatures[stack[-1]]<temperatures[i+1]:
#                     temperatures[stack[-1]] = i+1-stack[-1]
#                     stack.pop()
#             else:
#                 stack.append(i)
#         if stack:
#             for i in stack:
#                 temperatures[i]=0
#         temperatures[-1] = 0
#         return temperatures


s = Solution()
print(s.dailyTemperatures([73,74,75,71,69,72,76,73]))
print(s.dailyTemperatures([30,40,50,60]))
print(s.dailyTemperatures([30,60,90]))
print(s.dailyTemperatures([30,30,30,30,30,30,30,30,30,30,30,30,30]))
