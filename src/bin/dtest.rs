extern crate doubly;

use doubly::DoublyLinkedList;

fn main() {
    let mut nums: DoublyLinkedList<u32> = DoublyLinkedList::new_empty();

    for x in 1..100 { nums.push_back(x); }

    println!("Element 56 is: {}", nums[56]);
}
