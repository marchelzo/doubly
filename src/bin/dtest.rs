extern crate doubly;

use doubly::DoublyLinkedList;

fn main() {
    let mut nums: DoublyLinkedList<u32> = DoublyLinkedList::new_empty();

    for x in 1..100 { nums.push_back(x); println!("{}", x); }

    let mut vec: Vec<u32> = vec![];

    println!("\n\n ====== ADDED NUMBERS ======\n");

    while let Some(x) = nums.pop_front() {
        println!("{}", x);
    }

    println!("{} nums are left", nums.len());

    nums.insert(0, 5);
    nums.insert(1, 6);
    nums.insert(2, 8);
    nums.insert(2, 7);

    while let Some(x) = nums.pop_front() {
        println!("{}", x);
    }

    assert_eq!(0, nums.len());

    for x in 1..10 { nums.push_back(x); }

    nums.remove(3);

    while let Some(x) = nums.pop_front() {
        println!("{}", x);
    }

    for x in 1..100 { nums.push_front(x); }

    //println!("{}", nums.pop_front().unwrap());
    
    println!("{}", nums[0]);

    let mut other_nums = DoublyLinkedList::new_singleton(5u32);

    for x in 1..20 { other_nums.push_front(x); }

    nums.concat(other_nums);

    println!("Concatenated lists:");

    while let Some(x) = nums.pop_front() {
        println!("{}", x);
    }
}
