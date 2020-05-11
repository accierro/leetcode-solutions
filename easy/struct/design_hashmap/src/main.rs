// Design a HashMap without using any built-in hash table libraries.

// To be specific, your design should include these functions:

// put(key, value) : Insert a (key, value) pair into the HashMap. If the value already exists in the HashMap, update the value.
// get(key): Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key.
// remove(key) : Remove the mapping for the value key if this map contains the mapping for the key.

// Example:

// MyHashMap hashMap = new MyHashMap();
// hashMap.put(1, 1);
// hashMap.put(2, 2);
// hashMap.get(1);            // returns 1
// hashMap.get(3);            // returns -1 (not found)
// hashMap.put(2, 1);          // update the existing value
// hashMap.get(2);            // returns 1
// hashMap.remove(2);          // remove the mapping for 2
// hashMap.get(2);            // returns -1 (not found)

// Note:

// All keys and values will be in the range of [0, 1000000].
// The number of operations will be in the range of [1, 10000].
// Please do not use the built-in HashMap library.

struct Solution;

struct MyHashMap {
    bucket: [i32; 1000000],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    /** Initialize your data structure here. */
    fn new() -> Self {
        MyHashMap {
            bucket: [-1; 1000000],
        }
    }
    /** value will always be non-negative. */
    fn put(&mut self, key: i32, value: i32) {
        self.bucket[key as usize] = value;
    }
    /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
    fn get(&self, key: i32) -> i32 {
        self.bucket[key as usize]
    }
    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    fn remove(&mut self, key: i32) {
        self.bucket[key as usize] = -1;
    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */

fn main() {
    let mut obj = MyHashMap::new();
    obj.put(1, 10);
    let ret_2: i32 = obj.get(1);
    assert_eq!(ret_2, 10);
    obj.remove(1);
    assert_eq!(obj.get(1), -1);
}
