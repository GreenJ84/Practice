# You are given a string s. We want to partition the string into as many parts as possible so that each letter appears in at most one part.
# Note that the partition is done so that after concatenating all the parts in order, the resultant string should be s.
# Return a list of integers representing the size of these parts.

from typing import List


class Solution:
    def partitionLabels(self, s: str) -> List[int]:
        # Variable for the partition start
        # Variable to runthe array
        run = start = 0
        # Varibale for the answer
        ans = []
        # Enumerate the string
        for idx, item in enumerate(s):
            # Your runner either stays where it is at 
            # OR moves to the last occurance of the current character
            run = max(run, s.rfind(item))
            # If the runner gets aligned with the current index
            if idx == run:
                # Add the partition length to your answer
                ans.append(idx - start +1)
                # Start a new partition
                start = idx + 1
        
        return ans
