# Hash Map
Hash maps provide a data structure which maps keys to values.

## Functions
Hash maps provide the follow functions for interacting:
  - new() - Creates a new empty hash map
  - insert(key, value) - Inserts the value at key, returning the previous value if there is one
  - remove(key) - Removes the value located at key in the hash map and returns it if it exists
  - get_mut(key) - Returns a mutable reference to the value located at key if it exists
  - get(key) - Returns a reference to the value located at key if it exists
  - len() - Returns the current number elements stored in the hash map
  - iter() - Returns an iterator over the elements of the hash map
  - iter_mut() - Returns a mutable iterator over the elements of the hash map