class KthLargest:

    def __init__(self, k: int, nums: List[int]):
        

    def add(self, val: int) -> int:
        if self.nums[self.k] > val:
            if self.nums[self.k -1] > val:
                return self.nums[self.k -1]
            else:
                return val
        else:
            return self.nums[self.k]



# Your KthLargest object will be instantiated and called as such:
# obj = KthLargest(k, nums)
# param_1 = obj.add(val)
