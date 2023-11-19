// You are given an integer array nums. A number x is lonely when it appears only once, and no adjacent numbers (i.e. x + 1 and x - 1) appear in the array.

// Return all lonely numbers in nums. You may return the answer in any order.

import java.util.*;

class Solution {
    public List<Integer> findLonely(int[] nums) {
        HashMap<Integer, Integer> allUnique = new HashMap<Integer, Integer>();
        Arrays.stream(nums)
            .boxed()
            .forEach(n -> allUnique.put(
                n, allUnique.getOrDefault(n, 0) + 1
            ));
        System.gc();
        List<Integer> lonely = new ArrayList<Integer>();
        for (int num: nums){
            if (
                // If the number appears only once
                (allUnique.get(num) == 1) &&
                // and If not adjacents present
                (allUnique.get(num - 1) == null && allUnique.get(num + 1) == null)
            ){
                lonely.add(num);
            }
        }
        return lonely;
    }
}

class FindAllLonelyNumbersInTheArray {
    public static void main(String[] args) {
        Solution s = new Solution();

        List<Integer> test1 = s.findLonely(new int[]{10,6,5,8});
        System.out.println(test1);
        assert test1.size() == 2;
        assert test1.get(0) == 10;
        assert test1.get(1) == 8;

        List<Integer> test2 = s.findLonely(new int[]{1,3,5,3});
        System.out.println(test2);
        assert test2.size() == 2;
        assert test2.get(0) == 1;
        assert test2.get(1) == 5;
    }
}