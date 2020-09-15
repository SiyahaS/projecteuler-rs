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

#[allow(dead_code)]
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
