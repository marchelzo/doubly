extern crate doubly;

use doubly::DoublyLinkedList;

fn main() {
    let mut nums: DoublyLinkedList<u32> = DoublyLinkedList::new_empty();

    for x in 1..100 { nums.push_back(x); println!("{}", x); }

    println!("\n\n ====== ADDED NUMBERS ======\n");

    while let Some(x) = nums.pop_front() {
        println!("{}", x);
    }

}
