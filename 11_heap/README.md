# Heap Problems with Patterns

| File | Pattern(s) |
|------|------------|
| [0_Implement_Maxheap_MinHeap_arrays_recursion.py](0_Implement_Maxheap_MinHeap_arrays_recursion.py) | Heap / Array Manipulation / Recursion |
| [1_Sort_Array_using_heap_sort.py](1_Sort_Array_using_heap_sort.py) | Heap Sort / Sorting |
| [2_Maximum_all_subarrays_size_k.py](2_Maximum_all_subarrays_size_k.py) | Heap / Sliding Window / Max Heap |
| [3_k_largest_element_array.py](3_k_largest_element_array.py) | Heap / Min Heap |
| [4_Kth_smallest_largest_element_unsorted_array.py](4_Kth_smallest_largest_element_unsorted_array.py) | Heap / QuickSelect / Min & Max Heap |
| [5_Merge_k_Sorted_Arrays.py](5_Merge_k_Sorted_Arrays.py) | Min Heap / Priority Queue |
| [6_Merge_2_Binary_Max_Heaps.py](6_Merge_2_Binary_Max_Heaps.py) | Heap / Array Merging |
| [7_Kth_largest_sum_continuous_subarrays.py](7_Kth_largest_sum_continuous_subarrays.py) | Min Heap / Subarray Sum |
| [8_Reorganize_String.py](8_Reorganize_String.py) | Max Heap / Greedy |
| [9_Merge_K_sorted_linked_lists.py](9_Merge_K_sorted_linked_lists.py) | Min Heap / Linked List Merging |
| [10_Smallest_range_in_K_lists.py](10_Smallest_range_in_K_lists.py) | Min Heap / Sliding Window |
| [11_Median_stream_Integers.py](11_Median_stream_Integers.py) | Heap / Two Heaps (Min-Max) |
| [12_Is_Binary_Tree_Heap.py](12_Is_Binary_Tree_Heap.py) | Tree Traversal / Heap Property Validation |
| [13_Minimum_Cost_of_ropes.py](13_Minimum_Cost_of_ropes.py) | Greedy / Min Heap |
| [14_Convert_BST_to_Min_Max_Heap.py](14_Convert_BST_to_Min_Max_Heap.py) | Tree Conversion / Heap Construction |
| [15_Convert_Min_Heap_Max_Heap.py](15_Convert_Min_Heap_Max_Heap.py) | Heap Construction / Reordering |
| [16_Rearrange_characters.py](16_Rearrange_characters.py) | Max Heap / Greedy |
| [17_Minimum_sum.py](17_Minimum_sum.py) | Min Heap / Greedy |

---

# Python `heapq` Module Summary

Python’s `heapq` provides built-in methods to efficiently use heaps, especially min-heaps. Common methods include:

### Heap Creation and Manipulation
- `heapify()`, `heappush()`, `heappop()`, `heappushpop()`, `heapreplace()`

### Element Access
- `nlargest()`, `nsmallest()`, `merge()`, `_heapify_max()` (for max-heaps)

Useful in:
- Priority queues
- Scheduling
- Online median
- Merging sorted streams
- Greedy optimizations


# Python heapq Module

Python's `heapq` module provides functions to create and manipulate heaps. Heaps are binary trees for which every parent node has a value less than or equal to any of its children. This makes it suitable for applications such as priority queues and sorting algorithms.

## Methods

### Heap Creation:

1. **heapify(heap)**:
   - Converts a list into a heap in-place.
   - Time Complexity: O(n), where n is the number of elements in the list.

2. **heappush(heap, item)**:
   - Pushes a new element onto the heap while maintaining the heap invariant.
   - Time Complexity: O(log n), where n is the number of elements in the heap.

3. **heappop(heap)**:
   - Removes and returns the smallest element from the heap.
   - Time Complexity: O(log n), where n is the number of elements in the heap.

4. **heappushpop(heap, item)**:
   - Pushes a new element onto the heap and then pops and returns the smallest element.
   - Equivalent to calling `heappush()` followed by `heappop()`, but more efficient.
   - Time Complexity: O(log n), where n is the number of elements in the heap.

5. **heapreplace(heap, item)**:
   - Pops and returns the smallest element from the heap, then pushes a new element onto the heap.
   - More efficient than calling `heappop()` followed by `heappush()`.
   - Time Complexity: O(log n), where n is the number of elements in the heap.

### Heap Access:

6. **heappushpop(heap, item)**:
   - Returns the smallest element from the heap without removing it.
   - Time Complexity: O(1).

7. **nlargest(n, iterable)**:
   - Returns the n largest elements from the iterable.
   - Time Complexity: O(n + k log n), where n is the length of the iterable and k is the number of elements returned.

8. **nsmallest(n, iterable)**:
   - Returns the n smallest elements from the iterable.
   - Time Complexity: O(n + k log n), where n is the length of the iterable and k is the number of elements returned.

### Miscellaneous:

9. **merge(*iterables, key=None, reverse=False)**:
   - Merges multiple sorted inputs into a single sorted output.
   - Time Complexity: O(n log k), where n is the total number of elements and k is the number of input iterables.

10. **_heapify_max(heap)**:
    - Custom method to convert a list into a max-heap in-place.
    - Utilizes `heapify()` and negates values in the list to simulate a max-heap.
"""
