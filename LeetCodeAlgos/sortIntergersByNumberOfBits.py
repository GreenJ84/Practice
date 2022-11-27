# You are given an integer array arr. Sort the integers in the array in ascending order by the number of 1's in their binary representation and in case of two or more integers have the same number of 1's you have to sort them in ascending order.
# Return the array after sorting it.

from typing import List


class Solution:
    def sortByBits(self, arr: List[int]) -> List[int]:
        count = []
        for n in arr:
            count.append([bin(n)[2:].count('1'), n])
        count.sort()
        return [num for b, num in count]


s = Solution()
print(s.sortByBits([0,1,2,3,4,5,6,7,8]))
print(s.sortByBits([1024,512,256,128,64,32,16,8,4,2,1]))
