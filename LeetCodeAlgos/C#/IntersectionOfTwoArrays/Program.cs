// Given two integer arrays nums1 and nums2, return an array of their intersection. 
// Each element in the result must be unique and you may return the result in any order.



int[] Intersection(int[] nums1, int[] nums2) {
  HashSet<int> nums1Set = new(nums1);
  List<int> ans = [];
  
  foreach (int num2 in new HashSet<int>(nums2)){
      if (nums1Set.Contains(num2)){
          ans.Add(num2);
      }
  }
  
  return ans.ToArray();
}

int[] result1 = Intersection([1,2,2,1], [2,2]);
Console.WriteLine("Intersection 1: " + string.Join(", ", result1));

int[] result2 = Intersection([4,9,5], [9,4,9,8,4]);
Console.WriteLine("Intersection 2: " + string.Join(", ", result2));

