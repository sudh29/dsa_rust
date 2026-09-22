# User function Template for python3
class Solution:
    def solve(self, n, p, a, b, d):
        # Create a dictionary to store the connections and their diameters
        # print(n,p,a,b,d)
        connections = {}
        for i, j, k in zip(a, b, d):
            connections[i] = (j, k)
        # print(connections)

        # Identify tanks and taps
        tanks = set(a) - set(b)
        tanks = sorted(tanks)
        # print(tanks)

        # Find pairs of tanks and taps with minimum diameter pipes
        pairs = []
        for tank in tanks:
            tap, min_diameter = connections[tank]
            while tap in set(b):
                if (
                    tap not in connections
                ):  # Check if tap is not found in the connections
                    break
                min_diameter = min(min_diameter, connections[tap][1])
                tap = connections[tap][0]
            pairs.append([tank, tap, min_diameter])

        return pairs


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        n, p = map(int, input().strip().split())
        a = []
        b = []
        d = []
        for i in range(p):
            x, y, z = map(int, input().strip().split())
            a.append(x)
            b.append(y)
            d.append(z)

        ob = Solution()
        ans = ob.solve(n, p, a, b, d)
        print(len(ans))
        for i in ans:
            print(str(i[0]) + " " + str(i[1]) + " " + str(i[2]))


# } Driver Code Ends
