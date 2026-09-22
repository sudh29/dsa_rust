class Solution:
    def sort012(self, arr, n):
        # code here
        """dic={}
        for i in arr:
            if i not in dic:
                dic[i]=1
            else:
                dic[i]+=1
        #print(dic)
        temp=[]
        for i, v in dic.items():
            for j in range(v):
                temp.append(i)
        #print(temp)
        for i in range(n):
            arr[i]=temp[i]"""

        """zero=arr.count(0)
        one=arr.count(1)
        two=arr.count(2)
        #print(zero,one,two)
        for i in range(n):
            if zero>0:
                arr[i]=0
                zero-=1
            elif zero==0 and one>0:
                arr[i]=1
                one-=1
            elif zero==0 and one==0 and two>0:
                arr[i]=2
                two-=1"""

        low = 0
        high = n - 1
        mid = 0
        while mid <= high:
            if arr[mid] == 0:
                arr[mid], arr[low] = arr[low], arr[mid]
                low += 1
                mid += 1
            elif arr[mid] == 1:
                mid += 1
            else:
                arr[mid], arr[high] = arr[high], arr[mid]
                high -= 1

        return arr
