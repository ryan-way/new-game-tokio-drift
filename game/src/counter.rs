pub fn count(value: usize) -> usize {
    value + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let value = 1;
        assert_eq!(2, count(value));
    }
}
