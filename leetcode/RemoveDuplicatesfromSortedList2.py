# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

def deleteDuplicates(head: ListNode) -> ListNode:
    # Input: 1->1->2->3->3
    # Output: 2
    if head is None:
        return None
    node = head
    while True:
        child = node.next
        if node is None or child is None:
            break
        if node.val != child.val:
            node = child
        else:
            child = child.next
    return head
