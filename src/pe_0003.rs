// MIT License

// Copyright (c) 2020 Yakup Türkan

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
use std::convert::TryFrom;
use primal::Primes;

#[allow(dead_code)]
pub fn largest_prime_factor(number: u64) -> u64 {
    let mut primes = Primes::all();
    let mut n = number;
    let mut largest_prime: u64 = 0;

    let mut prime = u64::try_from(primes.next().unwrap()).unwrap();
    while n != 1  {
        largest_prime = if n % prime == 0 {
            n /= prime;
            prime
        } else {
            prime = u64::try_from(primes.next().unwrap()).unwrap();
            largest_prime
        };
    }

    if largest_prime == 0 { number } else { largest_prime }
}

#[cfg(test)]
mod tests {
    use super::largest_prime_factor;

    #[cfg(feature = "pe_0003")]
    #[test]
    fn largest_prime_factor_of_13_195() {
        assert_eq!(29, largest_prime_factor(13_195));
    }

    #[cfg(feature = "pe_0003")]
    #[test]
    fn largest_prime_factor_of_600_851_475_143 () {
        assert_eq!(6857, largest_prime_factor(600_851_475_143));
    }
}
