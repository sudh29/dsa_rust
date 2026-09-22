use std::collections::{HashMap, VecDeque};

pub fn first_non_repeating_stream(stream: &str) -> String {
    let mut counts = HashMap::new();
    let mut q = VecDeque::new();
    let mut res = String::new();

    for c in stream.chars() {
        *counts.entry(c).or_insert(0) += 1;
        q.push_back(c);

        while let Some(&front) = q.front() {
            if counts[&front] > 1 {
                q.pop_front();
            } else {
                break;
            }
        }
        if let Some(&front) = q.front() {
            res.push(front);
        } else {
            res.push('#');
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_repeating_stream() {
        assert_eq!(first_non_repeating_stream("aabc"), "a#bb");
        assert_eq!(first_non_repeating_stream("zz"), "z#");
    }
}
