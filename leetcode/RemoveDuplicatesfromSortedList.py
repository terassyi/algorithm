# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

def deleteDuplicates(head: ListNode) -> ListNode:
    # Input: 1->1->2->3->3
    # Output: 1->2->3
    if head is None:
        return None
    node = head
    while True:
        if node is None or node.next is None:
            break
        child = node.next
        while True:
            if child is None:
                node.next = child
                break
            if child.val == node.val:
                child = child.next
            else:
                node.next = child
                break
        node = node.next
    return head
