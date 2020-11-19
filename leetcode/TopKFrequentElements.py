class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        d = dict()
        for e in nums:
            if e in d:
                d[e] += 1
            else:
                d[e] = 1
        
        sorted_dict = sorted(d.items(), key=lambda x:-x[1])
        return [p[1] for p in sorted_dict][:k]
