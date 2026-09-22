def calculate_work_units(N, demands):
    total_work_units = 0
    net_demand = 0

    for demand in demands:
        net_demand += demand
        total_work_units += abs(net_demand)

    return total_work_units


# Main function
def main():
    while True:
        N = int(input())
        if N == 0:
            break

        demands = list(map(int, input().split()))
        work_units = calculate_work_units(N, demands)
        print(work_units)


if __name__ == "__main__":
    main()
