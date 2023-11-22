// Write a class that allows getting and setting key-value pairs, however a time until expiration is associated with each key.

// The class has three public methods:

// set(key, value, duration): accepts an integer key, an integer value, and a duration in milliseconds. Once the duration has elapsed, the key should be inaccessible. The method should return true if the same un-expired key already exists and false otherwise. Both the value and duration should be overwritten if the key already exists.

// get(key): if an un-expired key exists, it should return the associated value. Otherwise it should return -1.

// count(): returns the count of un-expired keys.

class TimeLimitedCache {
  constructor() { 
    this.cache = {};
  }
};

TimeLimitedCache.prototype.set = function (key, value, duration) {
  if (Object.keys(this.cache).includes(key.toString())) { 
    data = this.cache[key];
    this.cache[key] = [value, new Date().getTime() + duration];
    if (data[1] < new Date().getTime()) {
      return false;
    } else {
      return true;
    }
  }
  this.cache[key] = [value, new Date().getTime() + duration];
  return false;
};

TimeLimitedCache.prototype.get = function(key) {
  if (Object.keys(this.cache).includes(key)) { 
    data = this.cache[key];
    console.log(data, new Date().getTime())
    if (data[1] < new Date().getTime()) {
      delete this.cache[key];
      return -1;
    } else {
      return data[0];
    }
  } else {
    return -1;
  }
};

TimeLimitedCache.prototype.count = function() {
  return Object.entries(this.cache).filter(entry => entry[1][1] > new Date().getTime()).length;
};