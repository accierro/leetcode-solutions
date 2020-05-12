// Design a HashSet without using any built-in hash table libraries.

// To be specific, your design should include these functions:

// add(value): Insert a value into the HashSet.
// contains(value) : Return whether the value exists in the HashSet or not.
// remove(value): Remove a value in the HashSet. If the value does not exist in the HashSet, do nothing.

// Example:

// MyHashSet hashSet = new MyHashSet();
// hashSet.add(1);
// hashSet.add(2);
// hashSet.contains(1);    // returns true
// hashSet.contains(3);    // returns false (not found)
// hashSet.add(2);
// hashSet.contains(2);    // returns true
// hashSet.remove(2);
// hashSet.contains(2);    // returns false (already removed)

// Note:

// All values will be in the range of [0, 1000000].
// The number of operations will be in the range of [1, 10000].
// Please do not use the built-in HashSet library.

struct MyHashSet {
    buckets: [i32; 1000000],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    /** Initialize your data structure here. */
    fn new() -> Self {
        MyHashSet {
            buckets: [-1; 1000000],
        }
    }

    fn add(&mut self, key: i32) {
        self.buckets[key as usize] = 0;
    }

    fn remove(&mut self, key: i32) {
        self.buckets[key as usize] = -1;
    }
    /** Returns true if this set contains the specified element */
    fn contains(&self, key: i32) -> bool {
        self.buckets[key as usize] == 0
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */

fn main() {
    let mut obj = MyHashSet::new();
    obj.add(1);
    obj.remove(1);
    let ret_3: bool = obj.contains(1);
    assert_eq!(ret_3, false);
}
