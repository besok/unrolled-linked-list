//! # Unrolled linked list
//! An [`wiki`] is a linear data structure that is a variant on the linked list.
//! Instead of just storing 1 element at each node, unrolled linked lists store an entire array at each node.
//!
//! Unrolled linked lists combine the advantages of the array (small memory overhead) with the benefits of linked lists (fast insertion and deletion) to produce vastly better performance.
//! By storing multiple elements at each node, unrolled linked lists effectively spread out the overhead of linked lists across multiple elements.
//! So, if an unrolled linked list stores an array of 4 elements at each node, its spreading the linked list overhead (pointers) across those 4 elements.
//!
//! The true benefits of the unrolled linked list come in the form of caching. The unrolled linked list takes advantage of this when it comes to indexing.
//!
//! # Example
//! Let's suppose we have a following json:
//! ```rust
//!  use unrolled_linked_list::UnrolledLinkedList;
//!
//! fn main(){
//!   let mut list = UnrolledLinkedList::new();
//!   list.insert(0, 1);
//!   list.push(2);
//!   list.push(3);
//!   list.insert(3,4);
//!   if let Some(four) =  list.pop() { println!(" should be {}", four)}
//!
//!   let one_opt = list.get(0);
//!   let one_mut_opt = list.get_mut(0);
//!
//!   list.remove(0);
//!
//!   for el in list.iter(){
//!     println!("elem {}",el);
//!   }
//!
//! }
//! ```
//
//! [`wiki`]: https://en.wikipedia.org/wiki/Unrolled_linked_list/

use std::ptr::NonNull;
use std::fmt::{Display, Formatter, Debug};
use std::fmt;

pub mod iters;

/// The unrolled linked list. The list that acts like a linked list but has the node structure inside.
pub struct UnrolledLinkedList<T> {
    len: usize,
    cap: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}


impl<T> Display for UnrolledLinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "unrolled linked list: len:{}, cap:{}", self.len, self.cap)
    }
}

impl<T: fmt::Debug> Debug for UnrolledLinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<T> Default for UnrolledLinkedList<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> UnrolledLinkedList<T> {
    /// The default initiation, setting the size of node to 8.
    /// # Examples
    ///
    /// ```
    /// use unrolled_linked_list::UnrolledLinkedList;
    ///
    /// let list: UnrolledLinkedList<u32> = UnrolledLinkedList::new();
    /// ```
    pub fn new() -> Self {
        UnrolledLinkedList::with_capacity(8)
    }
    /// Capacity defines the size of the node.
    /// # Examples
    ///
    /// ```
    /// use unrolled_linked_list::UnrolledLinkedList;
    ///
    /// let list: UnrolledLinkedList<u32> = UnrolledLinkedList::with_capacity(4);
    /// ```
    pub fn with_capacity(cap: usize) -> Self {
        UnrolledLinkedList {
            cap,
            len: 0,
            head: None,
            tail: None,
        }
    }
}

impl<T> UnrolledLinkedList<T> {
    /// Adds an element last in the list.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use unrolled_linked_list::UnrolledLinkedList;
    /// let mut dl = UnrolledLinkedList::new();
    ///
    /// dl.push(2);
    /// assert_eq!(dl.pop().unwrap(), 2);
    ///
    /// dl.push(1);
    /// assert_eq!(dl.pop().unwrap(), 1);
    /// ```
    pub fn push(&mut self, el: T) {
        match (self.head, self.tail) {
            (_, Some(mut node)) | (Some(mut node), None) => {
                unsafe {
                    let node = node.as_mut();
                    if node.is_full(self.cap) {
                        self.tail = Some(node.split_and_push(el));
                    } else { node.data.push(el); }
                }
            }
            (None, None) => {
                let mut node = Box::new(Node::new());
                node.data.push(el);
                self.head = Some(Box::leak(node).into())
            }
        }
        self.len += 1;
    }
    /// Adds an element last in the list.
    /// # Panics
    /// Panics if `index > len`.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use unrolled_linked_list::UnrolledLinkedList;
    /// let mut dl = UnrolledLinkedList::new();
    ///
    /// dl.insert(0,0);
    /// assert_eq!(dl.pop().unwrap(), 0);
    /// ```
    pub fn insert(&mut self, index: usize, el: T) {
        if index > self.len {
            panic!("index {} should be less or equal the len {}", index, self.len)
        }

        if let (Some(mut node_ptr), start_idx) = self.find_node(index) {
            unsafe {
                let local_idx = index - start_idx;
                let node = node_ptr.as_mut();
                if node.is_full(self.cap) {
                    let next_node = node.split_and_insert(el, local_idx);
                    if self.tail.is_none() { self.tail = Some(next_node); }
                } else {
                    node.data.insert(local_idx, el);
                }
            }
        } else {
            let mut first_node = Box::new(Node::new());
            first_node.data.insert(index, el);
            self.head = Some(Box::leak(first_node).into())
        }
        self.len += 1;
    }
    /// removes the last element from the list and returns it.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use unrolled_linked_list::UnrolledLinkedList;
    /// let mut dl = UnrolledLinkedList::new();
    ///
    /// dl.insert(0,0);
    /// assert_eq!(dl.pop().unwrap(), 0);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            return match (self.head, self.tail) {
                (Some(mut f), None) => {
                    let first = f.as_mut();
                    if first.data.is_empty() {
                        None
                    } else {
                        self.len -= 1;
                        first.data.pop()
                    }
                }
                (_, Some(mut l)) => {
                    let last = l.as_mut();
                    let popped_value = last.data.pop();
                    if last.data.is_empty() { self.unlink_last(); }
                    self.len -= 1;
                    popped_value
                }
                _ => None
            };
        }
    }
    /// removes the custom element from the list accordign to the index and returns it.
    /// # Panics
    /// Panics if `index >= len`.
    /// # Examples
    ///
    /// ```
    ///
    /// use unrolled_linked_list::UnrolledLinkedList;
    /// let mut dl = UnrolledLinkedList::new();
    ///
    /// dl.insert(0,0);
    /// assert_eq!(dl.remove(0), 0);
    /// ```
    pub fn remove(&mut self, index: usize) -> T {
        if index >= self.len {
            panic!("index {} should be less then len {}", index, self.len)
        }
        unsafe {
            if let (Some(mut n), start_idx) = self.find_node(index) {
                let node = n.as_mut();
                let rem_element = node.data.remove(index - start_idx);
                node.steal_some(self.cap);
                self.len -= 1;
                rem_element
            } else {
                unreachable!("the node should exist");
            }
        }
    }
    /// retrieves the custom element from the list according to the index and returns it.
    /// # Examples
    ///
    /// ```
    ///
    /// use unrolled_linked_list::UnrolledLinkedList;
    /// let mut dl = UnrolledLinkedList::new();
    ///
    /// dl.insert(0,0);
    /// dl.insert(1,1);
    /// assert_eq!(dl.get(1), Some(&1));
    /// assert_eq!(dl.get(0), Some(&0));
    /// ```
    pub fn get(&self, index: usize) -> Option<&T> {
        unsafe {
            if let (Some(n), start_idx) = self.find_node(index) {
                (*n.as_ptr()).data.get(index - start_idx)
            } else { None }
        }
    }
    /// retrieves the custom element from the list according to the index and returns the mutable reference.
    /// # Examples
    ///
    /// ```
    ///
    /// use unrolled_linked_list::UnrolledLinkedList;
    /// let mut dl = UnrolledLinkedList::new();
    ///
    /// dl.insert(0,0);
    /// dl.insert(1,1);
    /// assert_eq!(dl.get_mut(1), Some(&mut 1));
    /// assert_eq!(dl.get_mut(0), Some(&mut 0));
    /// ```
    pub fn get_mut(&self, index: usize) -> Option<&mut T> {
        unsafe {
            if let (Some(n), start_idx) = self.find_node(index) {
                let node = &mut *n.as_ptr();
                node.data.get_mut(index - start_idx)
            } else { None }
        }
    }

    /// Returns `true` if the `UnrolledLinkedList` is empty.
    ///
    /// This operation should compute in *O*(1) time.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the length of the `UnrolledLinkedList`.
    ///
    /// This operation should compute in *O*(1) time.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Removes all elements from the `LinkedList`.
    ///
    /// This operation should compute in *O*(*n*) time.
    pub fn clear(&mut self) {
        *self = Self::with_capacity(self.cap);
    }

    /// Returns `true` if the `LinkedList` contains an element equal to the
    /// given value.
    pub fn contains(&self, x: &T) -> bool
        where
            T: PartialEq<T>,
    {
        self.iter().any(|e| e == x)
    }
}

impl<T> UnrolledLinkedList<T> {
    #[inline]
    unsafe fn unlink_last(&mut self) {
        if let Some(mut f) = self.head {
            let first = f.as_mut();
            if first.next == self.tail {
                first.unlink_next();
                self.tail = None;
            } else if let Some(mut p) = self.tail.and_then(|mut e| e.as_mut().prev) {
                let prev_node = p.as_mut();
                prev_node.unlink_next();
                self.tail = Some(p);
            }
        }
    }
    fn find_node(&self, idx: usize) -> (Option<NonNull<Node<T>>>, usize) {
        let mut shift = 0;
        let mut next_node = self.head;

        unsafe {
            while next_node.is_some() {
                if let Some(n) = next_node {
                    let node = n.as_ref();
                    let shift_end = shift + node.data.len();
                    if idx >= shift && idx <= shift_end {
                        return (Some(n), shift);
                    }
                    shift = shift_end;
                    next_node = node.next;
                }
            }
        }
        (None, 0)
    }
}

struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    data: Vec<T>,
}

impl<T> Node<T> {
    fn new() -> Self {
        Node {
            next: None,
            prev: None,
            data: vec![],
        }
    }

    unsafe fn unlink_next(&mut self) {
        if let Some(next) = self.next {
            let old_next = Box::from_raw(next.as_ptr());
            let new_next = old_next.next;
            if let Some(mut new) = new_next {
                new.as_mut().prev = NonNull::new(self as *mut Node<T>);
            }
            self.next = new_next;
        }
    }
    unsafe fn split(&mut self, mut next: NonNull<Node<T>>) {
        let len = self.data.len();
        self.link_next(next);
        next.as_mut().data = self.data.split_off(len / 2);
    }

    fn is_full(&self, cap: usize) -> bool {
        self.data.len() >= cap
    }
    #[inline]
    fn link_next(&mut self, mut next: NonNull<Node<T>>) {
        unsafe {
            if let Some(mut old_next) = self.next {
                old_next.as_mut().prev = Some(next);
                next.as_mut().next = Some(old_next);
            }
            self.next = Some(next);
            next.as_mut().prev = NonNull::new(self as *mut Node<T>);
        }
    }

    #[inline]
    unsafe fn steal_some(&mut self, cap: usize) {
        if self.data.len() < cap / 2 {
            if let Some(mut n) = self.next {
                let next = n.as_mut();
                if self.data.len() + next.data.len() >= cap {
                    let diff = cap / 2 - self.data.len();
                    let mut right = next.data.split_off(diff);
                    self.data.append(&mut next.data);
                    next.data.clear();
                    next.data.append(&mut right);
                } else {
                    self.data.append(&mut next.data);
                    self.unlink_next();
                }
            }
        }
    }
    #[inline]
    unsafe fn split_and_push(&mut self, el: T) -> NonNull<Node<T>> {
        let mut next_node = Box::leak(Box::new(Node::new())).into();
        self.split(next_node);
        next_node.as_mut().data.push(el);
        next_node
    }
    #[inline]
    unsafe fn split_and_insert(&mut self, el: T, idx: usize) -> NonNull<Node<T>> {
        let mut next_node = Box::leak(Box::new(Node::new())).into();
        self.split(next_node);
        let data_len = self.data.len();
        if idx > data_len {
            next_node.as_mut().data.insert(idx - data_len, el);
        } else {
            self.data.insert(idx, el);
        }
        next_node
    }
}


#[cfg(test)]
mod tests {
    use crate::UnrolledLinkedList;

    #[test]
    fn push_test() {
        let mut list = UnrolledLinkedList::with_capacity(4);
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);
        list.push(6);
        list.push(7);
        list.push(8);
        list.push(9);
        list.push(10);
        list.push(11);
        list.push(12);
        list.push(13);

        unsafe {
            let vec = list.tail.unwrap().as_ref().data.clone();
            assert_eq!(vec, vec![11, 12, 13]);
        }
    }

    #[test]
    fn pop_test() {
        let mut list = UnrolledLinkedList::with_capacity(4);

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);

        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));

        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);
        list.push(6);
        list.push(7);
        list.push(8);

        assert_eq!(list.pop(), Some(8));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(5));

        list.push(5);
        list.push(6);
        list.push(7);
        list.push(8);
        list.push(9);


        assert_eq!(list.pop(), Some(9));
        assert_eq!(list.pop(), Some(8));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert!(list.is_empty());

        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        assert_eq!(list.pop(), Some(4));
        list.push(5);
        assert_eq!(list.pop(), Some(5));
        list.push(6);
        list.push(7);
        list.push(8);
        assert_eq!(list.pop(), Some(8));
    }

    #[test]
    fn remove_test() {
        let mut list = UnrolledLinkedList::with_capacity(4);


        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);

        assert_eq!(list.remove(0), 1);
        assert_eq!(list.remove(0), 2);
        assert_eq!(list.remove(0), 3);
        assert_eq!(list.remove(0), 4);

        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);

        assert_eq!(list.remove(3), 4);
        assert_eq!(list.remove(2), 3);
        assert_eq!(list.remove(1), 2);
        assert_eq!(list.remove(0), 1);

        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);
        list.push(6);
        list.push(7);
        list.push(8);
        list.push(9);

        assert_eq!(list.remove(0), 1);
        assert_eq!(list.remove(0), 2);
        assert_eq!(list.remove(0), 3);
        assert_eq!(list.remove(0), 4);
        assert_eq!(list.remove(0), 5);
        assert_eq!(list.remove(0), 6);
        assert_eq!(list.remove(0), 7);
        assert_eq!(list.remove(0), 8);
        assert_eq!(list.remove(0), 9);
    }

    #[test]
    fn remove_loop_test() {
        let mut list = UnrolledLinkedList::new();
        for el in (0..1000).into_iter() {
            list.push(el);
        }
        for _ in (0..1000).into_iter() {
            let _ = list.remove(0);
        }
        assert!(list.is_empty())
    }

    #[test]
    fn insert_test() {
        let mut list = UnrolledLinkedList::with_capacity(4);
        list.insert(0, 1);
        list.insert(0, 2);
        list.insert(0, 3);
        list.insert(0, 4);
        list.insert(0, 5);
        list.insert(0, 6);
        list.insert(0, 7);

        unsafe {
            let vec = list.tail.unwrap().as_ref().data.clone();
            assert_eq!(vec, vec![2, 1]);
            let vec = list.head.unwrap().as_ref().data.clone();
            assert_eq!(vec, vec![7, 6, 5]);
        }
    }

    #[test]
    fn insert2_test() {
        let mut list = UnrolledLinkedList::with_capacity(4);
        list.insert(0, 0);
        list.insert(1, 1);
        assert_eq!(list.get(1), Some(&1));
        assert_eq!(list.get(0), Some(&0));
    }

    #[test]
    fn find_node_test() {
        let mut list = UnrolledLinkedList::with_capacity(4);
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);
        list.push(6);
        list.push(7);
        list.push(8);
        list.push(9);
        list.push(10);
        list.push(11);
        list.push(12);
        list.push(13);

        unsafe {
            let (node, start_idx) = list.find_node(5);
            let data = node.expect("").as_ref().data.clone();
            assert_eq!(data, vec![5, 6]);
            assert_eq!(5 - start_idx, 1);
        }
    }

    #[test]
    fn get_test() {
        let mut list = UnrolledLinkedList::with_capacity(4);
        list.push(1);
        list.push(1);
        list.push(1);
        list.push(4);
        list.push(1);

        assert_eq!(list.get(3), Some(&4));
        assert_eq!(list.get_mut(4), Some(&mut 1));
    }
}
