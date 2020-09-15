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
