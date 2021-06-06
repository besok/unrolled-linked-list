use std::ptr::NonNull;
use std::marker::PhantomData;
use std::borrow::BorrowMut;
use std::ops::{Deref, DerefMut};
use std::fmt::{Debug, Formatter, Display};
use std::fmt;

pub struct UnrolledLinkedList<T> {
    len: usize,
    cap: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}

pub struct Iter<'a, T> {
    len: usize,
    index: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    marker: PhantomData<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if let Some(n) = self.head {
            unsafe {
                let node = &*n.as_ptr();
                let elem = node.data.get(self.index);
                if self.index + 1 >= node.data.len() {
                    self.index = 0;
                    self.head = node.next;
                } else {
                    self.index += 1;
                }
                self.len -= 1;
                elem
            }
        } else { None }
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    #[inline]
    fn last(mut self) -> Option<&'a T> {
        unsafe {
            match (self.head, self.tail) {
                (Some(n), None) | (_, Some(n)) => (*n.as_ptr()).data.last(),
                _ => None
            }
        }
    }
}

pub struct IntoIter<T> {
    delegate: UnrolledLinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        if self.delegate.is_empty() { None } else { Some(self.delegate.remove(0)) }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.delegate.len, Some(self.delegate.len))
    }
}

impl<T> UnrolledLinkedList<T> {
    pub fn new() -> Self {
        UnrolledLinkedList::with_capacity(8)
    }
    pub fn with_capacity(cap: usize) -> Self {
        UnrolledLinkedList {
            cap,
            len: 0,
            head: None,
            tail: None,
        }
    }
}

impl<'a, T> UnrolledLinkedList<T> {
    pub fn iter(&self) -> Iter<'a, T> {
        Iter {
            len: self.len,
            index: 0,
            head: self.head,
            tail: self.tail,
            marker: Default::default(),
        }
    }
}

impl<T> IntoIterator for UnrolledLinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { delegate: self }
    }
}

impl<'a, T> IntoIterator for &'a UnrolledLinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}


impl<T> UnrolledLinkedList<T> {
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
                return rem_element;
            } else {
                unreachable!("the node should exist");
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> UnrolledLinkedList<T> {
    unsafe fn unlink_last(&mut self) {
        if let Some(mut f) = self.head {
            let first = f.as_mut();
            if first.next == self.tail {
                first.unlink_next();
                self.tail = None;
            } else {
                if let Some(mut p) = self.tail.and_then(|mut e| e.as_mut().prev) {
                    let prev_node = p.as_mut();
                    prev_node.unlink_next();
                    self.tail = Some(p);
                }
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
                    if idx >= shift && idx < shift_end {
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


    unsafe fn steal_some(&mut self, cap: usize) {
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

    unsafe fn split_and_push(&mut self, el: T) -> NonNull<Node<T>> {
        let mut next_node = Box::leak(Box::new(Node::new())).into();
        self.split(next_node);
        next_node.as_mut().data.push(el);
        next_node
    }
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
    use core::mem;
    use std::collections::LinkedList;
    use crate::structure::{UnrolledLinkedList, Node};
    use std::ptr::NonNull;

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
    fn iter_test() {
        let mut list = UnrolledLinkedList::with_capacity(4);
        for i in (1..20).into_iter() {
            list.push(i)
        }
        let mut idx = 1;
        for el in list.iter() {
            assert_eq!(el, &idx);
            idx += 1;
        }
    }

    #[test]
    fn into_iter_test() {
        let mut list = UnrolledLinkedList::with_capacity(4);
        for i in (1..20).into_iter() {
            list.push(i)
        }
        let mut idx = 1;
        for el in list.into_iter() {
            assert_eq!(el, idx);
            idx += 1;
        }
    }
}