def is_valid(x, y, N):
    return 1 <= x <= N and 1 <= y <= N

class Solution:
	#Function to find out minimum steps Knight needs to reach target position.
	def minStepToReachTarget(self, KnightPos, TargetPos, N):
		dx = [2, 2, -2, -2, 1, 1, -1, -1]
	        dy = [1, -1, 1, -1, 2, -2, 2, -2]

	        queue = []
	        queue.append([KnightPos[0], KnightPos[1], 0])
	        visited = [[False for _ in range(N + 1)] for _ in range(N + 1)]
	        visited[KnightPos[0]][KnightPos[1]] = True

	        while len(queue)>0:
	            x,y,dist = queue.pop(0)
	            if(x == TargetPos[0] and y == TargetPos[1]):
	                return dist
	            for i in range(8):
	                newX = x + dx[i]
	                newY = y + dy[i]

	                if is_valid(newX, newY, N) and not visited[newX][newY]:
	                    visited[newX][newY] = True
	                    queue.append([newX, newY, dist + 1])

#{
 # Driver Code Starts

T=int(input())
for i in range(T):
	N = int(input())
	KnightPos = list(map(int, input().split()))
	TargetPos = list(map(int, input().split()))
	obj = Solution()
	ans = obj.minStepToReachTarget(KnightPos, TargetPos, N)
	print(ans)

# } Driver Code Ends
