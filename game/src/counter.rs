pub fn count(value: i32) -> i32 {
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
