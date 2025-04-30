from memory import Pointer, UnsafePointer
from os import abort

alias ElementType = CollectionElement


fn main():
    node = Node(1)
    print(node.__str__())
    linkedlist = LinkedList(1, 2, 3)
    print(linkedlist.__str__())


@value
struct Node[
    T: ElementType,
]:
    alias NextNode = UnsafePointer[Self]
    var value: T
    var next: Self.NextNode

    fn __init__(
        out self,
        owned value: T,
    ):
        self.value = value
        self.next = Self.NextNode()
        #print("I am getting called", self.next.__bool__())

    fn __init__(
        out self,
        owned value: T,
        next: Optional[Self.NextNode],
    ):
        self.value = value^
        self.next = next.value() if next else Self.NextNode()
        #print("No I am getting called")

    fn __bool__(self) -> Bool:
        return True

    fn __str__[
        ElementType: WritableCollectionElement
    ](self: Node[ElementType]) -> String:
        return String.write(self.value)


struct LinkedList[T: ElementType]:
    var head: Optional[Node[T]]
    var len: UInt

    fn __init__(out self):
        self.head = None
        self.len = 0

    fn __init__(out self, *elems: T):
        self = Self()
        self.append(elems)

    fn append(mut self, elems: VariadicListMem[T]):
        if len(elems) == 0:
            return
        next = 0
        var current: Node[T]
        if self.head is None:
            self.head = Optional(Node(elems[0]))
            #self.head = Node(elems[0])
            current = self.head.value()
            next = 1
            self.len += 1
            print("ok1")

        else:
            current = self.head.value()
            while current and current.next:
                current = current.next[]
            print("ok2")
        for i in range(next, len(elems)):
            node = Node(elems[i])
            #current.next = Node[T].NextNode.alloc(1)
            #current.next = UnsafePointer[Node[T]].alloc(1)
            current.next = UnsafePointer(to=node)
            #ptr_next = UnsafePointer(to=node)
            #if not current.next:
                #abort()
            #current.next.init_pointee_move(node)
            current = current.next[]
            self.len += 1
            print("ok3")

    fn __str__[
        ElementType: WritableCollectionElement
    ](self: LinkedList[ElementType]) -> String:
        if self.len == 0:
            return String("[]")
        else:
            print("ok4", self.len)
            s = String("[")
            var current = self.head.value()
            print("ok5")
            s.write(current.value)
            for i in range(1, self.len):
                print("ok6", s)
                curr = current.next[]
                print("ok66", s)
                if i < self.len-1 :
                    s.write(", ")
                s.write(curr.value)
                print("ok7", s)
                print("ok8")
                current = curr
            # s = s.rstrip()
            s.write("]")
            return s
