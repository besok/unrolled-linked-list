### Unrolled linked list
An [unrolled linked list](https://en.wikipedia.org/wiki/Unrolled_linked_list)  is a linear data structure that is a variant on the linked list. 
Instead of just storing 1 element at each node, unrolled linked lists store an entire array at each node.

Unrolled linked lists combine the advantages of the array (small memory overhead) with the benefits of linked lists (fast insertion and deletion) to produce vastly better performance. 
By storing multiple elements at each node, unrolled linked lists effectively spread out the overhead of linked lists across multiple elements. 
So, if an unrolled linked list stores an array of 4 elements at each node, its spreading the linked list overhead (pointers) across those 4 elements.

The true benefits of the unrolled linked list come in the form of caching. The unrolled linked list takes advantage of this when it comes to indexing.

### How to use

The dependency can be found as following:
``` unrolled-linked-list = 1.0.0 ```

Example:
```rust

use unrolled_linked_list::UnrolledLinkedList;

fn main(){
  let mut list = UnrolledLinkedList::new();
  list.insert(0, 1);
  list.push(2);
  list.push(3);
  list.insert(3,4);
  if let Some(four) =  list.pop() { println!(" should be {}", four)}
  
  let one_opt = list.get(0);
  let one_mut_opt = list.get_mut(0);

  list.remove(0);  

  for el in list.iter(){
    println!("elem {}",el);
  }    
 
}

```

#### Comparison with linked list and vec

For the details, see the folder benches. 
The numbers and results have the typical format for the library [criterion](https://bheisler.github.io/criterion.rs/book/faq.html).

The typical result example:
```
Benchmarking push/unrolled_linked_list
Benchmarking push/unrolled_linked_list: Warming up for 3.0000 s
Benchmarking push/unrolled_linked_list: Collecting 100 samples in estimated 5.0358 s (364k iterations)
Benchmarking push/unrolled_linked_list: Analyzing
push/unrolled_linked_list
                        time:   [14.078 us 14.776 us 15.527 us]
                        change: [+3.9244% +7.5033% +11.271%] (p = 0.00 < 0.05)
                        Performance has regressed.
```


| Operation | Description |  unrolled ll | vec | ll |
| --- | --- | --- | --- | --- |
| push | insert 100 elements to the end | 14.7 | 12.3 | 25.8 |
| pop | retrieve 100 elements from the end | 20.6 | 12.1 | 20.6 | 
| insert | insert 100 elements to the beginning | 11.9 | 12.7 | 20.2 | 
| insert_middle | insert 100 elements to the middle | 14.0 | 13.1 | 22.8(absent, was replaced to push) |
| get | insert 100 elements to the middle | 16.6 | 13.1 | 27.7(absent, was replaced to push) |
| remove | remove the third part of elements | 17.8 | 15.8 | 24.3 |
| iter | iter through the collection | 17.9 | 12.8 | 24.8 |
| iter_mut | mut iter through the collection | 22.6 | 30.4 | 33.2 |
| into_iter |  iter with possession through the collection | 21.6 | 12.9 | 25.8 |
