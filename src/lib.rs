// We've got "Run Doctests" here
/// ```rust compile_fail
/// let x = 5;
/// x += 1;
/// ```
///
/// ```rust
/// let x = 5;
/// x + 1;
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// But we don't have "Run Doctests" here
/// ```rust compile_fail
/// let x = 5;
/// x *= 2;
/// ```
///
/// ```rust compile_fail
/// let x = 5;
/// x *= 2;
/// ```
pub fn mul(left: u64, right: u64) -> u64 {
    left * right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
