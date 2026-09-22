n = len(s)
  lps_arr = [0] * n  # Initialize lps array to store lengths of longest prefix suffixes

  # Build lps_arr for the pattern
  i = 1
  j = 0
  while i < n:
    if s[i] == s[j]:
      lps_arr[i] = j + 1
      i += 1
      j += 1
    else:
      # If characters don't match, check if current prefix (j) has a proper prefix-suffix
      if j != 0:
        j = lps_arr[j - 1]  # Go back to the previous proper prefix-suffix
      else:
        lps_arr[i] = 0  # No proper prefix-suffix found, set lps to 0
        i += 1

  # The last element of lps_arr stores the length of longest proper prefix-suffix
  return lps_arr[-1]
