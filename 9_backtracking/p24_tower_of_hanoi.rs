pub fn tower_of_hanoi(n: u32, from: char, to: char, aux: char, moves: &mut Vec<(char, char)>) {
    if n == 0 {
        return;
    }
    tower_of_hanoi(n - 1, from, aux, to, moves);
    moves.push((from, to));
    tower_of_hanoi(n - 1, aux, to, from, moves);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hanoi() {
        let mut moves = Vec::new();
        tower_of_hanoi(3, 'A', 'C', 'B', &mut moves);
        assert_eq!(moves.len(), 7);
    }
}
