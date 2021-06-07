use std::ptr::NonNull;
use std::marker::PhantomData;
use crate::{UnrolledLinkedList, Node};

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
    pub fn iter_mut(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            len: self.len,
            index: 0,
            head: self.head,
            tail: self.tail,
            delegate: self
        }
    }

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
    fn last( self) -> Option<&'a T> {
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

pub struct IterMut<'a,T>{
    len: usize,
    index: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    delegate: &'a mut UnrolledLinkedList<T>,
}
impl<'a,T> Iterator for IterMut<'a,T>{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut n) = self.head {
            unsafe {
                let mut node = &mut *n.as_ptr();
                let elem = node.data.get_mut(self.index);
                if self.index + 1 >= n.as_ref().data.len() {
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
}


#[cfg(test)]
mod tests {
    use crate::UnrolledLinkedList;


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
    #[test]
    fn mut_iter_test() {
        let mut list = UnrolledLinkedList::with_capacity(4);
        for _ in (1..20).into_iter() {
            list.push(1)
        }
        for el in list.iter_mut() {
            assert_eq!(el, &mut 1);
        }
    }
}