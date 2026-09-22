# User function Template for python3
class Solution:
    # Function to find if there exists a triplet in the
    # array A[] which sums up to X.
    def find3Numbers(self, A, n, X):
        # for i in range(n):
        #     newsum=X-A[i]
        #     for j in range(i+1,n):
        #         newsum2=newsum-A[j]
        #         if newsum2 in A[j+1:]:
        #             return True
        # return False
        A.sort()
        for i in range(n):
            newsum = X - A[i]
            j = i + 1
            k = n - 1
            while j < k:
                if A[j] + A[k] > newsum:
                    k -= 1
                elif A[j] + A[k] < newsum:
                    j += 1
                else:
                    return True
        return False
