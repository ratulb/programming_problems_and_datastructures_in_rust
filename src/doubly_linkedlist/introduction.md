# Dubley linkedlist

Here we implement a doubly link with quite a rich set of APIs. We provide following apis:
- `push_front`
- `pop_front`
- `push_back`
- `pop_back`
- `delete`
- `iter` - Returns a double ended iterator. Stops returning values when pointers meet. Immutable,
the list remains intact.
- `inter_into` - Returns a double ended iterator that consumes the elements. List becomes empty, oncethe iterator is fully traversed.

Let's look at the `Node` and `List` definitions next.
-

