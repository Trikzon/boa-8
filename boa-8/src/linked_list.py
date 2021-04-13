class Node:
    def __init__(self, value, next):
        self.value: int = value
        self.next: Node = next


class LinkedList:
    def __init__(self):
        self.head: Node = None

    def push(self, address: int):
        self.head = Node(address, self.head)

    def pop(self) -> int:
        if self.head is None:
            return None
        else:
            result: Node = self.head
            self.head = result.next

            return result.value
