def productExceptSelf(nums, n):
    total_product = 1
    total_product2 = 1
    count_zeros = 0

    # Calculate the total product and total product excluding zero elements
    for i in range(n):
        total_product *= nums[i]
        if nums[i] != 0:
            total_product2 *= nums[i]
        if nums[i] == 0:
            count_zeros += 1

    res = []

    # Generate the result based on the number of zero elements
    for i in range(n):
        if count_zeros > 1:
            res.append(0)
        elif nums[i] != 0 and total_product == 0:
            res.append(0)
        elif nums[i] == 0:
            res.append(total_product2)
        else:
            res.append(total_product // nums[i])

    return res
