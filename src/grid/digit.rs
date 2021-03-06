/*
 * MIT License
 *
 * Copyright (c) 2021 Piotr Dobiech
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LocalizedDigit<'a> {
    pub digit: Cow<'a, Digit>,
    pub x: usize,
    pub y: usize,
}

impl<'a> LocalizedDigit<'a> {
    pub fn new(digit: Cow<'a, Digit>, x: usize, y: usize) -> Self {
        Self { digit, x, y }
    }

    pub fn from_owned(digit: Digit, x: usize, y: usize) -> Self {
        let digit = Cow::Owned(digit);
        Self::new(digit, x, y)
    }

    pub fn from_borrowed(digit: &'a Digit, x: usize, y: usize) -> Self {
        let digit = Cow::Borrowed(digit);
        Self::new(digit, x, y)
    }

    pub fn into_owned_tuple(self) -> (Digit, usize, usize) {
        (self.digit.into_owned(), self.x, self.y)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Digit {
    Known(u32),
    Unknown(UnknownDigit),
}

impl Default for Digit {
    fn default() -> Self {
        let digit = UnknownDigit::default();
        Digit::Unknown(digit)
    }
}

impl From<u32> for Digit {
    fn from(digit: u32) -> Self {
        match digit {
            0 => Digit::Unknown(UnknownDigit::default()),
            _ => Digit::Known(digit),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnknownDigit {
    pub possible_values: Vec<u32>,
}

impl Default for UnknownDigit {
    fn default() -> Self {
        let values = Vec::default();
        Self::new(values)
    }
}

impl UnknownDigit {
    pub fn new(possible_values: Vec<u32>) -> Self {
        Self { possible_values }
    }
}
