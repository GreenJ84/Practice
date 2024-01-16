// Implement the RandomizedSet class:
  // RandomizedSet() Initializes the RandomizedSet object.
  // bool insert(int val) Inserts an item val into the set if not present. Returns true if the item was not present, false otherwise.
  // bool remove(int val) Removes an item val from the set if present. Returns true if the item was present, false otherwise.
  // int getRandom() Returns a random element from the current set of elements (it's guaranteed that at least one element exists when this method is called). Each element must have the same probability of being returned.
// You must implement the functions of the class such that each function works in average O(1) time complexity.

class RandomizedSet {
  set;
  constructor() {
    this.set = [];
  }
  insert(val) { 
    if (this.set.includes(val)) return false;
    this.set.push(val);
    return true;
  }
  remove(val) { 
    if (this.set.includes(val)) {
      this.set.splice(this.set.indexOf(val), 1);
      return true;
    }
    return false;
  }
  getRandom() { 
    return this.set[Math.floor(Math.random() * 1006) % this.set.length];
  }
}

(function testRandomizedSet() {
  const randomizedSet = new RandomizedSet();
  if (!randomizedSet.insert(1)) throw new Error('Test failed');
  if (randomizedSet.remove(2)) throw new Error('Test failed');
  if (!randomizedSet.insert(2)) throw new Error('Test failed');
  console.log(randomizedSet.getRandom());
  if (!randomizedSet.remove(1)) throw new Error('Test failed');
  if (randomizedSet.insert(2)) throw new Error('Test failed');
  console.log(randomizedSet.getRandom());
  console.log("Test passed!!")
})();