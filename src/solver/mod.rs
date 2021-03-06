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
use std::time::{Duration, Instant};

use crate::grid::digit::Digit;
use crate::grid::{Grid, GRID_JOINT_SIZE};

pub type SolverResult = Result<Grid, SolverError>;
pub type SolverError = Cow<'static, str>;

pub struct Solver {
    grid: Grid,
}

impl Solver {
    pub fn new(grid: Grid) -> Self {
        Self { grid }
    }

    pub fn solve(mut self) -> SolverResult {
        let start = Instant::now();
        while !self.grid.is_solved() && start.elapsed() < Duration::from_secs(4) {
            self.set_possible_values();
            self.set_values();
        }
        Ok(self.grid)
    }

    fn set_values(&mut self) {
        for x in 0..GRID_JOINT_SIZE {
            for y in 0..GRID_JOINT_SIZE {
                let digit = self.grid.get_digit_mut(x, y);
                if let Digit::Unknown(unknown_digit) = digit {
                    let values = &unknown_digit.possible_values;
                    if values.len() == 1 {
                        let value = values.first().unwrap();
                        let known_digit = Digit::Known(*value);
                        *digit = known_digit;
                    }
                }
            }
        }
    }

    fn set_possible_values(&mut self) {
        let grid_clone = self.grid.clone();
        for x in 0..GRID_JOINT_SIZE {
            for y in 0..GRID_JOINT_SIZE {
                let digit = self.grid.get_digit_mut(x, y);
                if let Digit::Unknown(digit) = digit {
                    let neighbours = grid_clone.get_neighbour_digits(x, y);
                    let supplement = Self::get_complement(neighbours);
                    digit.possible_values = supplement;
                }
            }
        }
    }

    fn get_complement(neighbours: Vec<&Digit>) -> Vec<u32> {
        let mut complement = (1..=9).collect::<Vec<_>>();
        for neighbour in neighbours {
            if let Digit::Known(neighbour) = neighbour {
                complement.retain(|element| element != neighbour);
            }
        }
        complement
    }
}
