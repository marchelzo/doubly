extern crate doubly;

use doubly::DoublyLinkedList;

fn main() {
    test_len();
}

fn test_len() {
    let mut nums = DoublyLinkedList::new_singleton(4i32);

    for x in 0..100 { nums.push_front(x); }
    for x in 0..100 { nums.push_back(x); }

    //nums.insert(40,8);

    println!("LENGTH: {}", nums.len());

    nums.insert(196,56);

    //assert_eq!(nums.len(), 202);
}
