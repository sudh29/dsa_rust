# Heap Problems with Patterns

| File | Pattern(s) |
|------|------------|
| [p00_implement_maxheap_minheap_arrays_recursion.rs](p00_implement_maxheap_minheap_arrays_recursion.rs) | Heap / Array Manipulation / Recursion |
| [p01_sort_array_using_heap_sort.rs](p01_sort_array_using_heap_sort.rs) | Heap Sort / Sorting |
| [p02_maximum_all_subarrays_size_k.rs](p02_maximum_all_subarrays_size_k.rs) | Heap / Sliding Window / Max Heap |
| [p03_k_largest_element_array.rs](p03_k_largest_element_array.rs) | Heap / Min Heap |
| [p04_kth_smallest_largest_element_unsorted_array.rs](p04_kth_smallest_largest_element_unsorted_array.rs) | Heap / QuickSelect / Min & Max Heap |
| [p05_merge_k_sorted_arrays.rs](p05_merge_k_sorted_arrays.rs) | Min Heap / Priority Queue |
| [p06_merge_2_binary_max_heaps.rs](p06_merge_2_binary_max_heaps.rs) | Heap / Array Merging |
| [p07_kth_largest_sum_continuous_subarrays.rs](p07_kth_largest_sum_continuous_subarrays.rs) | Min Heap / Subarray Sum |
| [p08_reorganize_string.rs](p08_reorganize_string.rs) | Max Heap / Greedy |
| [p09_merge_k_sorted_linked_lists.rs](p09_merge_k_sorted_linked_lists.rs) | Min Heap / Linked List Merging |
| [p10_smallest_range_in_k_lists.rs](p10_smallest_range_in_k_lists.rs) | Min Heap / Sliding Window |
| [p11_median_stream_integers.rs](p11_median_stream_integers.rs) | Heap / Two Heaps (Min-Max) |
| [p12_is_binary_tree_heap.rs](p12_is_binary_tree_heap.rs) | Tree Traversal / Heap Property Validation |
| [p13_minimum_cost_of_ropes.rs](p13_minimum_cost_of_ropes.rs) | Greedy / Min Heap |
| [p14_convert_bst_to_min_max_heap.rs](p14_convert_bst_to_min_max_heap.rs) | Tree Conversion / Heap Construction |
| [p15_convert_min_heap_max_heap.rs](p15_convert_min_heap_max_heap.rs) | Heap Construction / Reordering |
| [p16_rearrange_characters.rs](p16_rearrange_characters.rs) | Max Heap / Greedy |
| [p17_minimum_sum.rs](p17_minimum_sum.rs) | Min Heap / Greedy |

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
