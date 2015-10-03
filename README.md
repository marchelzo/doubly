doubly
======

#### Doubly-linked lists in Rust

### Usage

```rust
extern crate doubly;

use doubly::DoublyLinkedList;

fn main() {
    let my_list: DoublyLinkedList<i32> = DoublyLinkedList::new_empty();

    for k in 1..999999 {
        my_list.push_back(k);
    }

    my_list[7] = 85;

    for k in my_list.iter().take(10) {
        println!("{}", k);
    }
}
```
