pub fn even_fibonacci_numbers(limit: u32) -> u32 {
    let mut fibonacci = (1, 2);
    let mut sum = 0;

    while fibonacci.1 < limit {
        fibonacci = (fibonacci.1, fibonacci.0 + fibonacci.1);
        match fibonacci.0 % 2 {
            0 => sum += fibonacci.0,
            _ => {}
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::even_fibonacci_numbers;

    #[cfg(feature = "pe_0002")]
    #[test]
    fn till_90() {
        assert_eq!(44, even_fibonacci_numbers(90))
    }

    #[cfg(feature = "pe_0002")]
    #[test]
    fn till_4_000_000() {
        assert_eq!(4613732, even_fibonacci_numbers(4_000_000))
    }
}
