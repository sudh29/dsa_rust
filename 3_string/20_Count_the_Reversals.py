def countRev(S):
    if len(S) % 2 != 0:
        return -1

    unbalanced_open, unbalanced_close = 0, 0

    # stack = []
    # for char in S:
    #     if char == '{':
    #         stack.append(char)
    #     else:
    #         if stack and stack[-1] == '{':
    #             stack.pop()
    #         else:
    #             stack.append(char)

    # while stack:
    #     if stack.pop() == '{':
    #         unbalanced_open += 1
    #     else:
    #         unbalanced_close += 1

    for char in S:
        if char == "{":
            unbalanced_open += 1
        else:
            if unbalanced_open > 0:
                unbalanced_open -= 1
            else:
                unbalanced_close += 1

    reversals = (unbalanced_open + 1) // 2 + (unbalanced_close + 1) // 2
    return reversals


# {
# Driver Code Starts
t = int(input())
for tc in range(t):
    s = input()
    print(countRev(s))

# Contributed By: Pranay Bansal

# } Driver Code Ends
