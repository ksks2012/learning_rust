use std::collections::VecDeque;
use std::collections::LinkedList;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::collections::BinaryHeap;

//  Vec<T>
fn test_vec() {
    let mut v1 = vec![];
    v1.push(1);
    v1.push(2);
    v1.push(3);
    assert_eq!(v1, [1, 2, 3]);
    assert_eq!(v1[1], 2);

    let mut v2 = vec![0; 10];
    v2.push(1);
    println!("{:?}", v2);

    let mut v3 = Vec::new();
    v3.push(4);
    v3.push(5);
    v3.push(6);
    // v3[4] -> error: index out of bounds
}

// VecDeque<T>
fn test_vecdeque() {
    let mut buf = VecDeque::new();
    buf.push_front(1);
    buf.push_front(2);
    assert_eq!(buf.get(0), Some(&2));
    assert_eq!(buf.get(1), Some(&1));
    buf.push_back(3);
    buf.push_back(4);
    buf.push_back(5);
    assert_eq!(buf.get(2), Some(&3));
    assert_eq!(buf.get(3), Some(&4));
    assert_eq!(buf.get(4), Some(&5));
}

// LinkedList<T>
fn test_linked_list() {
    let mut list1 = LinkedList::new();
    list1.push_back('a');
    let mut list2 = LinkedList::new();
    list2.push_back('b');
    list2.push_back('c');

    list1.append(&mut list2);
    println!("{:?}", list1); // ['a', 'b', 'c']
    println!("{:?}", list2); // []

    list1.pop_front();
    println!("{:?}", list1); // ['b', 'c']

    list1.push_back('e');
    println!("{:?}", list1); // ['b', 'c', 'e']

    list2.push_front('f');
    println!("{:?}", list2); // ['f']
}

// HashMap<K, V> and BTreeMap<K, V>
fn test_map() {
    // unordered
    let mut hmap = HashMap::new();
    hmap.insert(3, 'c');
    hmap.insert(1, 'b');
    hmap.insert(2, 'b');
    hmap.insert(5, 'e');
    hmap.insert(4, 'd');
    println!("{:?}", hmap); // {3: 'c', 1: 'b', 5: 'e', 2: 'b', 4: 'd'}
    
    // ordered
    let mut bmap = BTreeMap::new();
    bmap.insert(3, 'c');
    bmap.insert(1, 'b');
    bmap.insert(2, 'b');
    bmap.insert(5, 'e');
    bmap.insert(4, 'd');
    println!("{:?}", bmap); // {1: 'b', 2: 'b', 3: 'c', 4: 'd', 5: 'e'}
}

// HashSet<T> and BTreeSet<T>
fn test_set() {
    let mut hbooks = HashSet::new();
    hbooks.insert("A Song of Ice and Fire");
    hbooks.insert("The Emerald City");
    hbooks.insert("The Ordyssey");
    if !hbooks.contains("The Emerald City") {
        println!("We have {} books, but The Emerald City ain't one.", hbooks.len());
    }
    println!("{:?}", hbooks); // random

    let mut bbooks = BTreeSet::new();
    bbooks.insert("A Song of Ice and Fire");
    bbooks.insert("The Emerald City");
    bbooks.insert("The Ordyssey");
    println!("{:?}", bbooks); // ordered {"A Song of Ice and Fire", "The Emerald City", "The Ordyssey"}
}

// BinaryHeap<T>
fn test_heap() {
    let mut heap = BinaryHeap::new();
    assert_eq!(heap.peek(), None);
    let arr = [93, 80, 48, 53, 72, 30, 18, 36, 15, 35, 45];
    for &i in arr.iter() {
        heap.push(i);
    }
    assert_eq!(heap.peek(), Some(&93));
    // [93, 80, 48, 53, 72, 30, 18, 36, 15, 35, 45]
    println!("{:?}", heap);
}

fn main() {
    test_vec();
    test_vecdeque();
    test_linked_list();
    test_map();
    test_set();
    test_heap();
}
