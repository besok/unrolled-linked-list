use std::ptr::NonNull;
use std::marker::PhantomData;
use std::borrow::BorrowMut;
use std::ops::{Deref, DerefMut};
use std::fmt::{Debug, Formatter, Display};

struct UnrolledLinkedList<T> {
    len: usize,
    cap: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}

impl<T> UnrolledLinkedList<T> {
    fn new() -> Self {
        UnrolledLinkedList::with_capacity(8)
    }
    fn with_capacity(cap: usize) -> Self {
        UnrolledLinkedList {
            cap,
            len: 0,
            head: None,
            tail: None,
        }
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
    fn new_with_data(data: Vec<T>) -> Self {
        Node {
            next: None,
            prev: None,
            data,
        }
    }
}


impl<T> Node<T> {
    fn as_nonnull(&mut self) -> Option<NonNull<Node<T>>> {
        NonNull::new(self as *mut Node<T>)
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
            next.as_mut().prev = self.as_nonnull();
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
}