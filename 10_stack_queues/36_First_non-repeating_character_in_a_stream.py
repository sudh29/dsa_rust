from collections import deque


class Solution:
    def FirstNonRepeating(self, A):
        # Using an array to keep track of character counts
        count = [0] * 26
        index = 0
        ans = ""
        q = deque()  # Using deque for efficient popping from the front

        for ch in A:
            index = ord(ch) - ord("a")  # Convert character to index (0-25)
            count[index] += 1
            q.append(ch)

            while q:
                if count[ord(q[0]) - ord("a")] > 1:  # Check if the front is repeating
                    q.popleft()  # Remove it if it is
                else:
                    ans += q[0]  # Add the first non-repeating character to the answer
                    break
            else:
                ans += "#"  # If queue is empty, append '#'

        return ans


# Alternative implementation using unordered_map equivalent
from collections import defaultdict


class SolutionAlt:
    def FirstNonRepeating(self, A):
        ans = ""
        q = deque()  # Queue to track non-repeating characters
        m = defaultdict(int)  # Using defaultdict for character count

        for ch in A:
            m[ch] += 1  # Increment character count
            if m[ch] == 1:
                q.append(ch)  # Add it to queue if it's first occurrence

            while q and m[q[0]] != 1:  # Remove repeating characters from the front
                q.popleft()

            if not q:
                ans += "#"  # Append '#' if no non-repeating characters
            else:
                ans += q[0]  # Append the front of the queue

        return ans


# Example usage
solution = Solution()
print(solution.FirstNonRepeating("aabcc"))  # Output: "a#b#"
