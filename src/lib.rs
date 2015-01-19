use std::mem;
use std::ptr;
use std::cell::Cell;
use std::num::SignedInt;
use std::ops::Index;
use std::ops::IndexMut;

struct Node<T> {
    prev: *mut Node<T>,
    value: T,
    next: *mut Node<T>
}

pub struct DoublyLinkedList<T> {
    current: Cell<*mut Node<T>>,
    first:   *mut Node<T>,
    last:    *mut Node<T>,
    index:   Cell<isize>,
    length:  usize
}

impl<T> DoublyLinkedList<T> {
    pub fn new_empty() -> DoublyLinkedList<T> {
        DoublyLinkedList {
            current: Cell::new(ptr::null_mut()),
            first:   ptr::null_mut(),
            last:    ptr::null_mut(),
            index:   Cell::new(-1),
            length:  0
        }
    }

    pub fn new_singleton(v: T) -> DoublyLinkedList<T> {
        unsafe {
            let node = Node::new_on_heap(v);
            DoublyLinkedList {
                current: Cell::new(node),
                first:   node,
                last:    node,
                index:   Cell::new(0),
                length:  1
            }
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn first_mut(&mut self) -> Option<&mut T> {
        unsafe { self.first.as_mut().map(|n| { &mut n.value }) }
    }

    pub fn first(&self) -> Option<&T> {
        unsafe { self.first.as_ref().map(|n| { &n.value }) }
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        unsafe { self.last.as_mut().map(|n| { &mut n.value }) }
    }

    pub fn last(&self) -> Option<&T> {
        unsafe { self.last.as_ref().map(|n| { &n.value }) }
    }

    pub fn push_back(&mut self, val: T) {
        unsafe {
            if self.length == 0 {
                *self = DoublyLinkedList::new_singleton(val);
            } else {
                (*(self.last)).next = Node::new_on_heap(val);
                (*(*(self.last)).next).prev = self.last;
                self.last = (*(self.last)).next;
                self.length += 1;
            }
        }
    }

    pub fn push_front(&mut self, val: T) {
        unsafe {
            if self.length == 0 {
                *self = DoublyLinkedList::new_singleton(val);
            } else {
                (*(self.first)).prev = Node::new_on_heap(val);
                (*(*(self.first)).prev).next = self.first;
                self.first = (*(self.first)).prev;
                self.length += 1;
            }
        }
    }

    fn index(&self, i: usize) -> Option<&T> {
        unsafe {
            if i >= self.length {
                None
            } else {
                if self.length / 2 > i {
                    if (self.index.get() - i as isize).abs() > i as isize {
                        self.go_to_from_start(i);
                    } else {
                        self.go_to(i);
                    }
                } else {
                    if (self.index.get() - i as isize).abs() >= (self.length as isize - i as isize).abs() {
                        self.go_to_from_end(i);
                    } else {
                        self.go_to(i);
                    }
                }
                Some(&(*self.current.get()).value)
            }
        }
    }

    fn index_mut(&mut self, i: usize) -> Option<&mut T> {
        unsafe {
            if i >= self.length {
                None
            } else {
                if self.length / 2 > i {
                    if (self.index.get() - i as isize).abs() > i as isize {
                        self.go_to_from_start(i);
                    } else {
                        self.go_to(i);
                    }
                } else {
                    if (self.index.get() - i as isize).abs() >= (self.length as isize - i as isize).abs() {
                        self.go_to_from_end(i);
                    } else {
                        self.go_to(i);
                    }
                }
                Some(&mut (*self.current.get()).value)
            }
        }
    }

    fn go_to(&self, i: usize) {
        unsafe {
            while (i as isize) > self.index.get() {
                self.current.set((*self.current.get()).next);
                self.index.set(self.index.get() + 1);
            }
            while (i as isize) < self.index.get() {
                self.current.set((*self.current.get()).prev);
                self.index.set(self.index.get() - 1);
            }
        }
    }

    fn go_to_from_start(&self, i: usize) {
        self.current.set(self.first);
        self.index.set(0);
        self.go_to(i);
    }

    fn go_to_from_end(&self, i: usize) {
        self.current.set(self.last);
        self.index.set(self.length as isize - 1);
        self.go_to(i);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        unsafe {
            if self.length == 0 {
                None
            } else {
                let val = ptr::read_and_zero(&mut (*self.last).value);
                if self.current.get() == self.last {
                    self.current.set((*self.last).prev);
                    self.index.set(self.index.get() - 1);
                }
                self.length -= 1;
                let old_last = self.last.clone();
                self.last = (*self.last).prev;
                drop(mem::transmute::<_, Box<Node<T>>>(old_last));
                Some(val)
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            if self.length == 0 {
                None
            } else {
                let val = ptr::read_and_zero(&mut (*self.first).value);
                if self.current.get() == self.first {
                    self.current.set((*self.first).next);
                }
                self.length -= 1;
                let old_first = self.first;
                self.first = (*self.first).next;
                drop(mem::transmute::<_, Box<Node<T>>>(old_first));
                Some(val)
            }
        }
    }
}

impl<T> Index<usize> for DoublyLinkedList<T> {
    type Output = T;
    fn index(&self, i: &usize) -> &T {
        self.index(*i).unwrap()
    }
}

impl<T> IndexMut<usize> for DoublyLinkedList<T> {
    type Output = T;
    fn index_mut(&mut self, i: &usize) -> &mut T {
        self.index_mut(*i).unwrap()
    }
}

impl<T> Node<T> {
    fn new(v: T) -> Node<T> {
        Node {
            prev: ptr::null_mut(),
            value: v,
            next: ptr::null_mut()
        }
    }

    unsafe fn new_on_heap(v: T) -> *mut Node<T> {
        let node = Box::new(Node::new(v));
        let node: *mut Node<T> = mem::transmute(node);
        node
    }
}

#[cfg(test)]
mod test {
    use super::DoublyLinkedList;

    #[test]
    fn test_list() {
        let mut nums = DoublyLinkedList::new_singleton(5i32);
        assert_eq!(5i32, *nums.last().unwrap());
        assert_eq!(nums.len(), 1);
        nums.push_front(7i32);
        assert_eq!(nums.len(), 2);
        assert_eq!(7i32, *nums.first().unwrap());
        nums.push_front(9i32);
        nums.push_front(6i32);
        assert_eq!(5i32, *nums.index(3).unwrap());
        nums.push_back(10i32);
        assert_eq!(5i32, *nums.index(3).unwrap());
  }
}
