# Definition for singly-linked list.
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

def detectCycle(head: ListNode) -> ListNode:
    if head is None:
        return None
    node = head
    l = list()
    while True:
        if node.next is None:
            return None
        if node.next in l:
            return node.next
        l.append(node)
        node = node.next
