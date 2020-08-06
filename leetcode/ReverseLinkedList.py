# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def reverseList(self, head: ListNode) -> ListNode:
        if head is None:
            return None
        node = head
        stack = [None]
        while True:
            if node is None:
                break
            stack.append(node.val)
            node = node.next
        
        reversedNode = ListNode(stack.pop())
        reversedNodeHead = reversedNode
        print(stack)
        while True:
            val = stack.pop()
            print(val)
            if val is None:
                reversedNode.next = None
                break
            child = ListNode(val)
            reversedNode.next = child
            reversedNode = reversedNode.next
        return reversedNodeHead
