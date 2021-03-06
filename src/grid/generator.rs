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

use crate::grid::digit::Digit;
use crate::grid::{Grid, GRID_SIZE, SUBGRID_SIZE};
use std::char::from_digit;

pub type GeneratorResult = Result<String, GeneratorError>;
pub type GeneratorError = Cow<'static, str>;

pub struct Generator;

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}

impl Generator {
    const SUBGRID_SPACER_LENGTH: usize = 2 * SUBGRID_SIZE - 1;
    const SPACER_LENGTH: usize =
        GRID_SIZE * Self::SUBGRID_SPACER_LENGTH + (GRID_SIZE - 1) * 5 + 2 * 3 - 2;

    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(&self, grid: &Grid) -> GeneratorResult {
        let mut result = String::new();
        self.add_upper_line_spacer(&mut result);
        for grid_y in 0..GRID_SIZE {
            self.add_spacer(&mut result);
            for subgrid_y in 0..SUBGRID_SIZE {
                result.push('|');
                result.push(' ');
                result.push(' ');
                for grid_x in 0..GRID_SIZE {
                    let subgrid = grid.get_subgrid_absolute(grid_x, grid_y);
                    for subgrid_x in 0..SUBGRID_SIZE {
                        let digit = subgrid.get_digit(subgrid_x, subgrid_y);
                        let digit_char = digit.to_char();
                        result.push(digit_char);
                        result.push(' ');
                    }
                    result.push(' ');
                    result.push('|');
                    result.push(' ');
                    result.push(' ');
                }
                result.push('\n');
            }
            self.add_line_spacer(&mut result);
        }
        Ok(result)
    }

    fn add_upper_line_spacer(&self, result: &mut String) {
        let line_spacer = "_".repeat(Self::SPACER_LENGTH);
        result.push('.');
        result.push_str(&line_spacer);
        result.push('.');
    }

    fn add_spacer(&self, result: &mut String) {
        result.push('\n');
        result.push('|');
        let spacer = " ".repeat(Self::SUBGRID_SPACER_LENGTH);
        for _ in 0..GRID_SIZE {
            result.push(' ');
            result.push(' ');
            result.push_str(&spacer);
            result.push(' ');
            result.push(' ');
            result.push('|');
        }
        result.push('\n');
    }

    fn add_line_spacer(&self, result: &mut String) {
        result.push('|');
        let line_spacer = "_".repeat(Self::SUBGRID_SPACER_LENGTH);
        for _ in 0..GRID_SIZE {
            result.push('_');
            result.push('_');
            result.push_str(&line_spacer);
            result.push('_');
            result.push('_');
            result.push('|');
        }
    }
}

impl Digit {
    fn to_char(&self) -> char {
        match self {
            Digit::Known(value) => from_digit(*value, 10).unwrap(),
            Digit::Unknown(_) => '0',
        }
    }
}
