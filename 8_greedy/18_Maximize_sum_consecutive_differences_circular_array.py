# User function Template for python3


def maxSum(arr, n):
    arr.sort()
    res = 0
    for i in range(n):
        res += abs(arr[i] - arr[n - 1 - i])
    return res

    # new_a = []
    # i=0
    # j=n-1
    # while i<j:
    #     new_a.append(arr[i])
    #     new_a.append(arr[j])
    #     i+=1
    #     j-=1
    # if len(new_a)!=n:
    #     new_a.insert(0,arr[i])
    # # print(new_a)
    # MaximumSum = 0
    # for i in range(0, n - 1):
    #     MaximumSum = MaximumSum + abs(new_a[i] - new_a[i + 1])

    # MaximumSum = MaximumSum + abs(new_a[0] - new_a[n-1])
    # return MaximumSum


# {
# Driver Code Starts
# Initial Template for Python 3


t = int(input())
for _ in range(0, t):
    n = int(input())
    # x=list(map(int,input().split()))
    # n=x[0]
    # k=x[1]
    arr = list(map(int, input().split()))
    ans = maxSum(arr, n)
    print(ans)
