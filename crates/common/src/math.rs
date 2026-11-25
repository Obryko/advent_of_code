/// Returns the greatest common divisor of two numbers.
pub const fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Returns the least common multiple of two numbers.
pub const fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gcd_works() {
        assert_eq!(gcd(54, 24), 6);
    }

    #[test]
    fn lcm_works() {
        assert_eq!(lcm(4, 6), 12);
    }
}
