# Definition for singly-linked list.
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

def hasCycle(head: ListNode) -> bool:
    if head is None:
        return False
    s = set()
    node = head
    while True:
        if node.next is None:
            return False
        if node.next in s:
            return True
        s.add(node)
        node = node.next
