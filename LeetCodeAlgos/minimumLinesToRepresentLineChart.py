# You are given a 2D integer array stockPrices where stockPrices[i] = [dayi, pricei] indicates the price of the stock on day dayi is pricei. A line chart is created from the array by plotting the points on an XY plane with the X-axis representing the day and the Y-axis representing the price and connecting adjacent points. 

from typing import List

from decimal import Decimal
class Solution:
    def minimumLines(self, stockPrices: List[List[int]]) -> int:
        if len(stockPrices) < 2:
            return 0
        if len(stockPrices) == 2:
            return 1
        stockPrices.sort(key=lambda x: x[0])
        def get_slope(first, second):
            [x1, y1] = first
            [x2, y2] = second
            return Decimal(y2 - y1) / Decimal(x2 - x1)

        currSlope = get_slope(stockPrices[0], stockPrices[1])
        lines = 1

        for i in range(1, len(stockPrices) - 1):
            slope = get_slope(stockPrices[i], stockPrices[i + 1])
            
            if abs(slope - currSlope) != 0 :
                lines += 1
                currSlope = slope
        return lines

s = Solution()
print(s.minimumLines([[1,7],[2,6],[3,5],[4,4],[5,4],[6,3],[7,2],[8,1]]))
print(s.minimumLines([[3,4],[1,2],[7,8],[2,3]]))
print(s.minimumLines([[1,1],[500000000,499999999],[1000000000,999999998]]))