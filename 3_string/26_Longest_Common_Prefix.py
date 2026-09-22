class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        if not strs:
            return ""
        n = len(strs)
        min_len = min(len(s) for s in strs)
        lcp = ""
        for i in range(min_len):
            current_char = strs[0][i]
            for s in strs:
                if s[i] != current_char:
                    return lcp
            lcp += current_char
        return lcp
