#User function Template for python3
class Solution:
    def smallestNumber (self, S, D):
        if 9*D < S: return -1
        ans = [0 for i in range(D)]
        for i in range(D-1,-1,-1):
            if S>9:
                ans[i] = '9'
                S-=9
            else:
                if i==0:
                    ans[i] = str(S)
                else:
                    ans[i] = str(S-1)
                    i-=1
                    while i>0:
                        ans[i] = '0'
                        i-=1
                    ans[i] = '1'
                    break
        return ''.join(ans)







#{
 # Driver Code Starts
#Initial Template for Python 3
if __name__ == '__main__':
    t = int (input ())
    for _ in range (t):
