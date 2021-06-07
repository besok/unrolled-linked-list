### Unrolled linked list
An [unrolled linked list](https://en.wikipedia.org/wiki/Unrolled_linked_list)  is a linear data structure that is a variant on the linked list. 
Instead of just storing 1 element at each node, unrolled linked lists store an entire array at each node.

Unrolled linked lists combine the advantages of the array (small memory overhead) with the benefits of linked lists (fast insertion and deletion) to produce vastly better performance. 
By storing multiple elements at each node, unrolled linked lists effectively spread out the overhead of linked lists across multiple elements. 
So, if an unrolled linked list stores an array of 4 elements at each node, its spreading the linked list overhead (pointers) across those 4 elements.

The true benefits of the unrolled linked list come in the form of caching. The unrolled linked list takes advantage of this when it comes to indexing.