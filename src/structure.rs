use std::ptr::NonNull;
use std::marker::PhantomData;
use std::borrow::BorrowMut;
use std::ops::{Deref, DerefMut};
use std::fmt::{Debug, Formatter};

struct UnrolledLinkedList<T> {
    len: usize,
    node_cap: usize,
    first: Option<NonNull<Node<T>>>,
    last: Option<NonNull<Node<T>>>,
}

struct Node<T> {
    next: Option<*mut Node<T>>,
    prev: Option<*mut Node<T>>,
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
    fn split(&mut self, cap: usize) -> Box<Node<T>> {
        let mut next_node = Box::new(Node::new());
        self.link_next(next_node.as_ref_mut());
        let next_vec = self.data.split_off(cap / 2);
        next_node.data = next_vec;
        next_node
    }


    fn push(&mut self, el: T, cap: usize) -> Option<*mut Node<T>> {
        if self.data.len() < cap {
            self.data.push(el);
            Some(self.as_ref_mut())
        } else {
            self.split(cap).push(el, cap)
        }
    }

    fn link_next(&mut self, new_node: *mut Node<T>) {
        unsafe {
            if self.next.is_some() {
                let old_next = self.next.unwrap();
                (*old_next).prev = Some(new_node);
                (*new_node).next = Some(old_next);
            }
            (*new_node).prev = Some(self.as_ref_mut());
            self.next = Some(new_node);
        }
    }


    fn as_ref_mut(&mut self) -> *mut Node<T> {
        self as *mut Node<T>
    }
}


#[cfg(test)]
mod tests {
    use core::mem;
    use std::collections::LinkedList;
    use crate::structure::{UnrolledLinkedList, Node};
    use std::ptr::NonNull;


    #[test]
    fn link_next_test() {
        let mut node0 = Box::new(Node::new_with_data(vec![0]));
        let mut node2 = Box::new(Node::new_with_data(vec![2]));
        let mut node1 = Box::new(Node::new_with_data(vec![1]));


        unsafe {
            node0.link_next(node2.as_ref_mut());

            assert_eq!((*node0.next.unwrap()).data, &*node2.data);
            assert_eq!((*node2.prev.unwrap()).data, &*node0.data);

            node0.link_next(node1.as_ref_mut());
            assert_eq!((*node2.prev.unwrap()).data, &*node1.data);
            assert_eq!((*node1.prev.unwrap()).data, &*node0.data);
            assert_eq!(node0.prev, None);
        }
    }

    #[test]
    fn push_test() {
        let mut node0 = Box::new(Node::new());

        node0.push(1, 4);
        node0.push(2, 4);
        node0.push(3, 4);
        node0.push(4, 4);
        assert!(node0.next.is_none());
        assert_eq!(node0.data, vec![1, 2, 3, 4]);
        node0.push(5, 4);
        assert_eq!(node0.data, vec![1, 2]);
        unsafe { assert_eq!( (*node0.next.unwrap()).data, vec![3, 4, 5])}
    }
}