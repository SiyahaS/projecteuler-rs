/// Finds sum of multiples of 3 and 5 till the given number
#[allow(dead_code)]
pub fn multiples_of_3_and_5(i: u32) -> u32 {
    (0..i).filter(|x| x % 3 == 0 || x % 5 == 0).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::multiples_of_3_and_5;

    #[cfg(feature = "pe_0001")]
    #[test]
    fn till_10() {
        assert_eq!(23, multiples_of_3_and_5(10))
    }
    
    #[cfg(feature = "pe_0001")]
    #[test]
    fn till_100() {
        assert_eq!(2318, multiples_of_3_and_5(100))
    }

    #[cfg(feature = "pe_0001")]
    #[test]
    fn till_1000() {
        assert_eq!(233168, multiples_of_3_and_5(1000))
    }
}
