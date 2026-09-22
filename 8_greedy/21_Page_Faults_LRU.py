# User function Template for python3


class Solution:
    def pageFaults(self, N, C, pages):
        arr = []
        page_fault = 0
        for i in range(N):
            page = pages[i]
            if page not in arr:
                if len(arr) == C:
                    arr.pop(0)
                arr.append(page)
                page_fault += 1
            else:
                arr.remove(page)
                arr.append(page)
        return page_fault


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        N = int(input())
        pages = input().split()
        for itr in range(N):
            pages[itr] = int(pages[itr])
        C = int(input())

        ob = Solution()
        print(ob.pageFaults(N, C, pages))

# } Driver Code Ends
