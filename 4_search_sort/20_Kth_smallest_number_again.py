def main():
    t = int(input())
    for _ in range(t):
        n, q = map(int, input().split())
        v = []

        for _ in range(n):
            x, y = map(int, input().split())
            v.append((x, y))

        v.sort()

        idx = 0
        for i in range(1, len(v)):
            if v[idx][1] >= v[i][0]:
                v[idx] = (v[idx][0], max(v[idx][1], v[i][1]))
            else:
                idx += 1
                v.append(v[i])  # Append new interval

        # Remove extra intervals from the list
        v = v[: idx + 1]

        for _ in range(q):
            k = int(input())
            ans = -1
            for i in range(idx + 1):
                if (v[i][1] - v[i][0] + 1) >= k:
                    ans = v[i][0] + k - 1
                    break
                else:
                    k -= v[i][1] - v[i][0] + 1

            print(ans)


if __name__ == "__main__":
    main()
