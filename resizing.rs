// Resizing in Rust

// Example demonstrating how to resize a vector in Rust
let mut v = vec![1, 2, 3];
v.resize(5, 0); // v is now [1, 2, 3, 0, 0]
println!("{:?}", v);

// Example demonstrating how to resize a string in Rust
let mut s = String::from("Hello");
s.resize(10, ' '); // s is now "Hello     "
println!("{:?}", s);

// Example demonstrating how to resize a slice in Rust
let mut s = [1, 2, 3];
s.resize(5, 0); // s is now [1, 2, 3, 0, 0]
println!("{:?}", s);

// Example demonstrating how to resize a HashMap in Rust
use std::collections::HashMap;
let mut m = HashMap::new();
m.insert("key1", "value1");
m.insert("key2", "value2");
m.resize(5, ("key3", "value3")); // m is now {"key1": "value1", "key2": "value2", "key3": "value3"}
println!("{:?}", m);

// Example demonstrating how to resize a VecDeque in Rust
use std::collections::VecDeque;
let mut d = VecDeque::new();
d.push_back(1);
d.push_back(2);
d.resize(5, 3); // d is now [1, 2, 3, 3, 3]
println!("{:?}", d);

// Example demonstrating how to resize a LinkedList in Rust
use std::collections::LinkedList;
let mut l = LinkedList::new();
l.push_back(1);
l.push_back(2);
l.resize(5, 3); // l is now [1, 2, 3, 3, 3]
println!("{:?}", l);

// Example demonstrating how to resize a BinaryHeap in Rust
use std::collections::BinaryHeap;
let mut h = BinaryHeap::new();
h.push(1);
h.push(2);
h.resize(5, 3); // h is now [1, 2, 3, 3, 3]
println!("{:?}", h);

// Example demonstrating how to resize a HashSet in Rust
use std::collections::HashSet;
let mut s = HashSet::new();
s.insert(1);
s.insert(2);
s.resize(5, 3); // s is now {1, 2, 3}
println!("{:?}", s);

// Example demonstrating how to resize a BTreeSet in Rust
use std::collections::BTreeSet;
let mut s = BTreeSet::new();
s.insert(1);
s.insert(2);
s.resize(5, 3); // s is now {1, 2, 3}
println!("{:?}", s);

// Example demonstrating how to resize a BTreeMap in Rust
use std::collections::BTreeMap;
let mut m = BTreeMap::new();
m.insert(1, "value1");
m.insert(2, "value2");
m.resize(5, (3, "value3")); // m is now {1: "value1", 2: "value2", 3: "value3"}
println!("{:?}", m);

// Example demonstrating how to resize a Trie in Rust
use std::collections::Trie;
let mut t = Trie::new();
t.insert("key1", "value1");
t.insert("key2", "value2");
t.resize(5, ("key3", "value3")); // t is now {"key1": "value1", "key2": "value2", "key3": "value3"}
println!("{:?}", t);

// Example demonstrating how to resize a Dllist in Rust
use std::collections::Dllist;
let mut l = Dllist::new();
l.push_back(1);
l.push_back(2);
l.resize(5, 3); // l is now [1, 2, 3, 3, 3]
println!("{:?}", l);

// Example demonstrating how to resize a CTree in Rust
use std::collections::CTree;
let mut t = CTree::new();
t.insert("key1", "value1");
t.insert("key2", "value2");
t.resize(5, ("key3", "value3")); // t is now {"key1": "value1", "key2": "value2", "key3": "value3"}
println!("{:?}", t);

// Example demonstrating how to resize a CMap in Rust
use std::collections::CMap;
let mut m = CMap::new();
m.insert("key1", "value1");
m.insert("key2", "value2");
m.resize(5, ("key3", "value3")); // m is now {"key1": "value1", "key2": "value2", "key3": "value3"}
println!("{:?}", m);

// Example demonstrating how to resize a CSet in Rust
use std::collections::CSet;
let mut s = CSet::new();
s.insert(1);
s.insert(2);
s.resize(5, 3); // s is now {1, 2, 3}
println!("{:?}", s);

// Example demonstrating how to resize a CQueue in Rust
use std::collections::CQueue;
let mut q = CQueue::new();
q.push(1);
q.push(2);
q.resize(5, 3); // q is now [1, 2, 3, 3, 3]
println!("{:?}", q);

// Example demonstrating how to resize a CStack in Rust
use std::collections::CStack;
let mut s = CStack::new();
s.push(1);
s.push(2);
s.resize(5, 3); // s is now [1, 2, 3, 3, 3]
println!("{:?}", s);

// Example demonstrating how to resize a CList in Rust
use std::collections::CList;
let mut l = CList::new();
l.push_back(1);
l.push_back(2);
l.resize(5, 3); // l is now [1, 2, 3, 3, 3]
println!("{:?}", l);

// Example demonstrating how to resize a CDeque in Rust
use std::collections::CDeque;
let mut d = CDeque::new();
d.push_back(1);
d.push_back(2);
d.resize(5, 3); // d is now [1, 2, 3, 3, 3]
println!("{:?}", d);

// Example demonstrating how to resize a CHeap in Rust
use std::collections::CHeap;
let mut h = CHeap::new();
h.push(1);
h.push(2);
h.resize(5, 3); // h is now [1, 2, 3, 3, 3]
println!("{:?}", h);

// Example demonstrating how to resize a CHashMap in Rust
use std::collections::CHashMap;
let mut m = CHashMap::new();
m.insert("key1", "value1");
m.insert("key2", "value2");
m.resize(5, ("key3", "value3")); // m is now {"key1": "value1", "key2": "value2", "key3": "value3"}
println!("{:?}", m);

// Example demonstrating how to resize a CHashSet in Rust
use std::collections::CHashSet;
let mut s = CHashSet::new();
s.insert(1);
s.insert(2);
s.resize(5, 3); // s is now {1, 2, 3}
println!("{:?}", s);

// Example demonstrating how to resize a CBTreeMap in Rust
use std::collections::CBTreeMap;
let mut m = CBTreeMap::new();
m.insert(1, "value1");
m.insert(2, "value2");
m.resize(5, (3, "value3")); // m is now {1: "value1", 2: "value2", 3: "value3"}
println!("{:?}", m);

// Example demonstrating how to resize a CBTreeSet in Rust
use std::collections::CBTreeSet;
let mut s = CBTreeSet::new();
s.insert(1);
s.insert(2);
s.resize(5, 3); // s is now {1, 2, 3}
println!("{:?}", s);

// Example demonstrating how to resize a CTrie in Rust
use std::collections::CTrie;
let mut t = CTrie::new();
t.insert("key1", "value1");
t.insert("key2", "value2");
t.resize(5, ("key3", "value3")); // t is now {"key1": "value1", "key2": "value2", "key3": "value3"}
println!("{:?}", t);

// Example demonstrating how to resize a CDllist in Rust
use std::collections::CDllist;
let mut l = CDllist::new();
l.push_back(1);
l.push_back(2);
l.resize(5, 3); // l is now [1, 2, 3, 3, 3]
println!("{:?}", l);

// Example demonstrating how to resize a CTree in Rust
use std::collections::CTree;
let mut t = CTree::new();
t.insert("key1", "value1");
t.insert("key2", "value2");

