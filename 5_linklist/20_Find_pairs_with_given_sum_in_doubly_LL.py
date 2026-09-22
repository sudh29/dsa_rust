class Solution:
    def findPairsWithGivenSum(
        self, target: int, head: Optional["Node"]
    ) -> List[List[int]]:
        firstnode = head
        lastnode = head
        while lastnode.next:
            lastnode = lastnode.next

        res = []
        while firstnode != lastnode:
            temp_sum = firstnode.data + lastnode.data
            if temp_sum > target:
                lastnode = lastnode.prev
            elif temp_sum < target:
                firstnode = firstnode.next
            else:
                res.append((firstnode.data, lastnode.data))
                firstnode = firstnode.next
                if firstnode == lastnode:
                    break
                lastnode = lastnode.prev
        return res
