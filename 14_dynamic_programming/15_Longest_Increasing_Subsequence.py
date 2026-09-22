if n == 0:
            return 0

        # dp array to store the smallest end elements of all increasing subsequences
        # with different lengths
        dp = []

        for num in a:
            pos = binary_search(dp, num)

            # If pos is equal to the length of dp, it means num is greater than
            # all elements in dp, hence we extend the longest subsequence found so far
            if pos == len(dp):
                dp.append(num)
            else:
                # Otherwise, replace the element at pos with num
                dp[pos] = num

        # The length of dp will be our answer
        return len(dp)
