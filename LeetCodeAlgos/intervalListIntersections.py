# You are given two lists of closed intervals, firstList and secondList, where firstList[i] = [starti, endi] and secondList[j] = [startj, endj]. Each list of intervals is pairwise disjoint and in sorted order.
# Return the intersection of these two interval lists.
# A closed interval [a, b] (with a <= b) denotes the set of real numbers x with a <= x <= b.
# The intersection of two closed intervals is a set of real numbers that are either empty or represented as a closed interval. For example, the intersection of [1, 3] and [2, 4] is [2, 3].

from typing import List


class Solution:
    def intervalIntersection(self, firstList: List[List[int]], secondList: List[List[int]]) -> List[List[int]]:
        if not firstList or not secondList:
            return []
        res = []
        f, s = 0, 0
        while f < len(firstList) and s < len(secondList):
            first = firstList[f]
            second = secondList[s]
            if first[0] > second[1]:
                s+=1
                continue
            elif first[1] < second[0]:
                f+=1
                continue
            else:
                res.append([max(first[0], second[0]), min(first[1], second[1])])
                if first[1] > second[1]:
                    s+=1
                else:
                    f+=1
        return res

s = Solution()
print(s.intervalIntersection([[0,2],[5,10],[13,23],[24,25]], [[1,5],[8,12],[15,24],[25,26]]))
print(s.intervalIntersection([[1,3],[5,9]],[]))
