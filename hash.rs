// hash.rs

// A hash table implementation in Rust

// Define a struct to represent a hash table
struct HashTable<K, V> {
    table: Vec<Vec<(K, V)>>,
    size: usize,
}

// Implement methods on the HashTable struct
impl<K, V> HashTable<K, V> {
    // Create a new hash table with the given size
    fn new(size: usize) -> Self {
        let table = vec![Vec::new(); size];
        HashTable { table, size }
    }

    // Insert a new key-value pair into the hash table
    fn insert(&mut self, key: K, value: V) -> Result<(), String> {
        let index = self.get_index(&key);
        for (k, v) in &mut self.table[index] {
            if k == &key {
                *v = value;
                return Ok(());
            }
        }
        self.table[index].push((key, value));
        Ok(())
    }

    // Get the value associated with a given key
    fn get(&self, key: &K) -> Option<&V> {
        let index = self.get_index(key);
        for (k, v) in &self.table[index] {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    // Remove a key-value pair from the hash table
    fn remove(&mut self, key: &K) -> Result<(), String> {
        let index = self.get_index(key);
        for i in 0..self.table[index].len() {
            if self.table[index][i].0 == *key {
                self.table[index].remove(i);
                return Ok(());
            }
        }
        Err("Key not found".to_string())
    }

    // Get the index of a given key in the hash table
    fn get_index(&self, key: &K) -> usize {
        let mut sum = 0;
        for c in key.to_string().chars() {
            sum += c as u8 as usize;
        }
        sum % self.size
    }
}

// Define a trait for types that can be hashed
pub trait Hash {
    // Get the hash value of a value
    fn hash(&self) -> u64;
}

// Implement the Hash trait for the String type
impl Hash for String {
    fn hash(&self) -> u64 {
        let mut sum = 0;
        for c in self.chars() {
            sum += c as u8 as u64;
        }
        sum
    }
}

// Implement the Hash trait for the u32 type
impl Hash for u32 {
    fn hash(&self) -> u64 {
        *self as u64
    }
}

// Implement the Hash trait for the i32 type
impl Hash for i32 {
    fn hash(&self) -> u64 {
        *self as u64
    }
}

// Implement the Hash trait for the f32 type
impl Hash for f32 {
    fn hash(&self) -> u64 {
        unsafe { std::mem::transmute::<f32, u32>(*self) as u64 }
    }
}

// Implement the Hash trait for the f64 type
impl Hash for f64 {
    fn hash(&self) -> u64 {
        unsafe { std::mem::transmute::<f64, u64>(*self) }
    }
}

// Implement the Hash trait for the bool type
impl Hash for bool {
    fn hash(&self) -> u64 {
        if *self { 1 } else { 0 }
    }
}

// Implement the Hash trait for the char type
impl Hash for char {
    fn hash(&self) -> u64 {
        *self as u64
    }
}

fn main() {
    let mut hash_table = HashTable::<String, String>::new(10);
    hash_table.insert("key1".to_string(), "value1".to_string()).unwrap();
    hash_table.insert("key2".to_string(), "value2".to_string()).unwrap();
    println!("{:?}", hash_table);
    println!("{:?}", hash_table.get(&"key1".to_string()));
    println!("{:?}", hash_table.get(&"key2".to_string()));
    println!("{:?}", hash_table.remove(&"key1".to_string()));
    println!("{:?}", hash_table);
}
