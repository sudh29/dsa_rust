 word_set = set(dictionary)

        # Memoization dictionary to store results of subproblems
        memo = {}

        def can_break(start):
            # If we reach the end of the string, return True
            if start == len(s):
                return True

            # If this subproblem has already been solved, return the result
            if start in memo:
                return memo[start]

            # Try every possible end position for the current substring
            for end in range(start + 1, len(s) + 1):
                if s[start:end] in word_set and can_break(end):
                    memo[start] = True
                    return True

            # If no valid segmentation found, mark this start as False
            memo[start] = False
            return False

        # Start the recursion from the beginning of the string
        return 1 if can_break(0) else 0
