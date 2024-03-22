// Implement the RandomizedSet class:
// RandomizedSet() Initializes the RandomizedSet object.
// bool insert(int val) Inserts an item val into the set if not present. Returns true if the item was not present, false otherwise.
// bool remove(int val) Removes an item val from the set if present. Returns true if the item was present, false otherwise.
// int getRandom() Returns a random element from the current set of elements (it's guaranteed that at least one element exists when this method is called). Each element must have the same probability of being returned.
// You must implement the functions of the class such that each function works in average O(1) time complexity.

public class RandomizedSet {
    private HashSet<int> set;
    private Random rand = new();

    public RandomizedSet() {
        set = new HashSet<int>();
    }
    
    public bool Insert(int val) {
        return set.Add(val);
    }
    
    public bool Remove(int val) {
        return set.Remove(val);
    }
    
    public int GetRandom() {
        return set.ToArray()[rand.Next(0, set.Count)];
    }
}


RandomizedSet obj = new RandomizedSet();
bool param_1 = obj.Insert(1);
bool param_2 = obj.Remove(2);
int param_3 = obj.GetRandom();
Console.WriteLine(param_1 + " " + param_2 + " " + param_3);