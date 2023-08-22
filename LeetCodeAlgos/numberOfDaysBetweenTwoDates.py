# Write a program to count the number of days between two dates.

# The two dates are given as strings, their format is YYYY-MM-DD as shown in the examples.

from datetime import datetime

class Solution:
    def daysBetweenDates(self, date1: str, date2: str) -> int:
        return abs((datetime.strptime(date1, '%Y-%m-%d') - datetime.strptime(date2, '%Y-%m-%d')).days)
    
s = Solution()
print(s.daysBetweenDates("2019-06-29", "2019-06-30"))
print(s.daysBetweenDates("2020-01-15", "2019-12-31"))