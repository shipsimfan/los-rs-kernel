# Map
Maps provide a data structure which automatically associates values with ids and stores those values.

## Functions
Maps provide the following functions for interacting with them:
  - new() - Creates a new empty map
  - insert(value) - Inserts a new value into the map and returns its id
  - remove(id) - Removes the value stored at id from the map and returns it if it exists
  - get_mut(id) - Returns a mutable reference to the value stored at id
  - get() - Returns a reference to the value stored at id
  - len() - Returns the current number of values stored by the map
  - iter() - Returns an iterator over the values of the map
  - iter_mut() - Returns a mutable iterator over the values of the map