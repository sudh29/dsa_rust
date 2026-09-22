class Solution:
    # Function to find the maximum number of activities that can
    # be performed by a single person.
    def activitySelection(self, n, start, end):
        meetings = []
        for i in range(n):
            meetings.append([start[i], end[i]])
        # print(meetings)
        sorted_meetings_end_time = sorted(meetings, key=lambda x: x[1])
        # print(sorted_meetings_end_time)
        max_count = 1
        i = 0
        for j in range(1, n):
            if sorted_meetings_end_time[i][1] < sorted_meetings_end_time[j][0]:
                max_count += 1
                i = j
        return max_count
