def main():
    # Read number of soldiers
    n = int(input())

    # Read soldier powers
    soldier_power = []
    for _ in range(n):
        temp = int(input())
        soldier_power.append(temp)

    # Sort soldier powers
    soldier_power.sort()

    # Read number of queries
    q = int(input())

    # Process each query
    for _ in range(q):
        bishnu_power = int(input())
        count = 0
        cumulative_strength = 0

        # Check each soldier's power against Bishnu's power
        for i in range(n):
            if bishnu_power >= soldier_power[i]:
                count += 1
                cumulative_strength += soldier_power[i]
            else:
                print(count, cumulative_strength)
                break
        else:
            # If loop completes without break (Bishnu can defeat all soldiers)
            print(count, cumulative_strength)


# Run the main function
if __name__ == "__main__":
    main()
