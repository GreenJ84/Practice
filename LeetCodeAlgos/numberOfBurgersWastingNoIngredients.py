# Given two integers tomatoSlices and cheeseSlices. The ingredients of different burgers are as follows:
    # Jumbo Burger: 4 tomato slices and 1 cheese slice.
    # Small Burger: 2 Tomato slices and 1 cheese slice.
# Return [total_jumbo, total_small] so that the number of remaining tomatoSlices equal to 0 and the number of remaining cheeseSlices equal to 0. If it is not possible to make the remaining tomatoSlices and cheeseSlices equal to 0 return [].

from typing import List


class Solution:
    def numOfBurgers(self, tomatoSlices: int, cheeseSlices: int) -> List[int]:
        # Eliminate usatisfactory ingredient pools
        if tomatoSlices < cheeseSlices * 2:
            return []
        elif tomatoSlices > cheeseSlices * 4:
            return []
        elif tomatoSlices % 2 == 1:
            return []
        
        # Get max jumbo burgers with the rest small
        jumbo = tomatoSlices // 4
        tomatoSlices = tomatoSlices % 4
        small = tomatoSlices // 2

        # If we dont have enough for min burgers will fail
        cheeseNeed = jumbo + small
        if cheeseSlices < cheeseNeed:
            return []
        cheeseSlices -= cheeseNeed

        while cheeseSlices > 0:
            # Leftover Cheese
            if jumbo == 0:
                return []
            jumbo -= 1
            small += 2
            cheeseSlices -= 1

        return [jumbo, small]
    
s = Solution()
print(s.numOfBurgers(16, 7))
print(s.numOfBurgers(17, 4))