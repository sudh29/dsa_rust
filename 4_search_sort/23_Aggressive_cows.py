def largest_min_distance(t, test_cases):
    results = []

    for _ in range(t):
        n, c = test_cases[_][:2]
        stall_location = test_cases[_][2]
        stall_location.sort()

        start = 1
        end = 10**9
        ans = 0

        while start <= end:
            mid = (start + end) // 2
            cow = 1
            prev = stall_location[0]

            for i in range(1, n):
                if stall_location[i] - prev >= mid:
                    cow += 1
                    prev = stall_location[i]
                    if cow == c:
                        break

            if cow == c:
                ans = mid
                start = mid + 1
            else:
                end = mid - 1

        results.append(ans)

    return results


# Example usage:
t = int(input("Enter number of test cases: "))
test_cases = []
for _ in range(t):
    n, c = map(int, input("Enter n and c: ").split())
    stall_location = list(map(int, input("Enter stall locations: ").split()))
    test_cases.append((n, c, stall_location))

results = largest_min_distance(t, test_cases)
for result in results:
    print(result)
