class Solution:
    def findLongestChain(self, pairs: List[List[int]]) -> int:
        pairs.sort(key=lambda x: x[1])
        current_end = float("-inf")
        max_chain_length = 0
        for pair in pairs:
            if current_end < pair[0]:
                current_end = pair[1]
                max_chain_length += 1
        return max_chain_length
