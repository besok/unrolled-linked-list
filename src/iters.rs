use std::ptr::NonNull;
use std::marker::PhantomData;
use crate::{UnrolledLinkedList, Node};
use std::fmt;

impl<'a, T> UnrolledLinkedList<T> {
    /// Provides a forward iterator.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use unrolled_linked_list::UnrolledLinkedList;
    ///
    /// let mut list: UnrolledLinkedList<u32> = UnrolledLinkedList::new();
    ///
    /// list.push(0);
    /// list.push(1);
    /// list.push(2);
    ///
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(&0));
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<'a, T> {
        Iter {
            len: self.len,
            index: 0,
            head: self.head,
            tail: self.tail,
            marker: Default::default(),
        }
    }

    /// Provides a forward mut iterator.
   ///
   /// # Examples
   ///
   /// ```
   ///
   /// use unrolled_linked_list::UnrolledLinkedList;
   ///
   /// let mut list: UnrolledLinkedList<u32> = UnrolledLinkedList::new();
   ///
   /// list.push(0);
   /// list.push(1);
   /// list.push(2);
   ///
   /// for element in list.iter_mut() {
   ///     *element += 10;
   /// }
   ///
   /// let mut iter = list.iter();
   /// assert_eq!(iter.next(), Some(&10));
   /// assert_eq!(iter.next(), Some(&11));
   /// assert_eq!(iter.next(), Some(&12));
   /// assert_eq!(iter.next(), None);
   /// ```
    pub fn iter_mut(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            len: self.len,
            index: 0,
            head: self.head,
            delegate: self
        }
    }

}
/// An iterator over the elements of a `UnrolledLinkedList`.
///
/// This `struct` is created by [`UnrolledLinkedList::iter()`]. See its
/// documentation for more.
pub struct Iter<'a, T> {
    len: usize,
    index: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    marker: PhantomData<&'a Node<T>>,
}

impl<T: fmt::Debug> fmt::Debug for Iter<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Iter").field(&self.len).finish()
    }
}
impl<T> Clone for Iter<'_, T> {
    fn clone(&self) -> Self {
        Iter { ..*self }
    }
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

/// An owning iterator over the elements of a `UnrolledLinkedList`.
///
/// This `struct` is created by the [`into_iter`] method on [`UnrolledLinkedList`]
/// (provided by the `IntoIterator` trait). See its documentation for more.
///
/// [`into_iter`]: UnrolledLinkedList::into_iter
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
impl<T: fmt::Debug> fmt::Debug for IntoIter<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IntoIter").field(&self.delegate).finish()
    }
}
impl<'a, T> IntoIterator for &'a UnrolledLinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// A mutable iterator over the elements of a `UnrolledLinkedList`.
///
/// This `struct` is created by [`UnrolledLinkedList::iter_mut()`].
/// See its documentation for more.
pub struct IterMut<'a,T>{
    len: usize,
    index: usize,
    head: Option<NonNull<Node<T>>>,
    delegate: &'a mut UnrolledLinkedList<T>,
}
impl<'a,T> Iterator for IterMut<'a,T>{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.head {
            unsafe {
                let node = &mut *n.as_ptr();
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
impl<T: fmt::Debug> fmt::Debug for IterMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IterMut").field(&self.delegate).field(&self.len).finish()
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