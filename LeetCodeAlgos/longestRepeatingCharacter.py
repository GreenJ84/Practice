# You are given a string s and an integer k. You can choose any character of the string and change it to any other uppercase English character. You can perform this operation at most k times.
# Return the length of the longest substring containing the same letter you can get after performing the above operations.

class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        start=maxWindow=0
        count={}

        for end in s:
            # Get a character count of values in window
            count[end]=count.get(end,0)+1

            # If we are over k used chars(totalChars - maxRepeated)>k
            while sum(count.values())-max(count.values())>k:
                # Drop front of window in count
                count[s[start]]-=1
                # Slide window
                start+=1
            # Check maximum window size
            maxWindow=max(maxWindow,sum(count.values()))
        return maxWindow

