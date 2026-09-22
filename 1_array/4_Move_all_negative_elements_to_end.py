class Solution:
        def segregateElements(self, arr, n):
        # Your code goes here
        # temp1=[i for i in arr if i<0]
        # temp2=[i for i in arr if i>=0]
        # temp=(temp2+temp1)
        # for i in range(n):
        #     arr[i]=temp[i]
        # return arr

        for i in range(n):
            if arr[i]>=0:
                pos=i
                break
        for i in range(n):
            if arr[i]<0:
                neg=i
                break
        while(pos<n and neg<n):
            if pos<neg:
                pos+=1
            else:
                arr[pos],arr[neg]=arr[neg],arr[pos]
                pos+=1
                neg+=1

        print(arr)


# Two pointer
def move_negatives(arr):
    left = 0
    right = len(arr) - 1
    while left < right:
        if arr[left] < 0:
            left += 1
        elif arr[right] >= 0:
            right -= 1
        else:
            arr[left], arr[right] = arr[right], arr[left]
            left += 1
            right -= 1
