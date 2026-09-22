#[derive(Debug, PartialEq, Eq)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
}

pub fn list_comprehension_squares(n: usize) -> Vec<usize> {
    (0..=n).map(|i| i * i).collect()
}

pub fn filter_strings(items: &[&str], prefix: &str) -> Vec<String> {
    items
        .iter()
        .filter(|s| s.starts_with(prefix))
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comp_and_user() {
        assert_eq!(list_comprehension_squares(4), vec![0, 1, 4, 9, 16]);
        let movies = ["acsc", "bsds", "gasxc", "gmovie"];
        assert_eq!(filter_strings(&movies, "g"), vec!["gasxc", "gmovie"]);
        let user = User {
            first_name: "Dave".into(),
            last_name: "Bowman".into(),
        };
        assert_eq!(user.first_name, "Dave");
    }
}
