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

use std::borrow::{Borrow, Cow};
use std::time::{Duration, Instant};

use crate::grid::digit::{Digit, LocalizedDigit};
use crate::grid::position::Position;
use crate::grid::{Grid, GRID_JOINT_SIZE, GRID_SIZE, SUBGRID_LENGTH, SUBGRID_SIZE};

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
        while !self.grid.is_solved() && start.elapsed() < Duration::from_secs(1) {
            self.set_possible_values();
            self.set_single_possible_values();
            self.set_possible_values();
            self.find_single_possible_solutions_for_subgrids();
            self.set_possible_values();
            self.find_single_possible_solutions_for_columns();
            self.set_possible_values();
            self.find_single_possible_solutions_for_rows();
        }
        Ok(self.grid)
    }

    fn find_single_possible_solutions_for_subgrids(&mut self) {
        for subgrid in &mut self.grid.subgrids {
            let digits = subgrid.localized_digits();
            Self::get_solutions(digits)
                .into_iter()
                .map(|solution| solution.into_owned_tuple())
                .for_each(|(digit, x, y)| subgrid.set_digit(x, y, digit));
        }
    }

    fn find_single_possible_solutions_for_columns(&mut self) {
        for x in 0..GRID_JOINT_SIZE {
            let column = self.grid.get_vertical_localized_digits(x);
            Self::get_solutions(column)
                .into_iter()
                .map(|solution| solution.into_owned_tuple())
                .for_each(|(digit, x, y)| self.grid.set_digit(x, y, digit));
        }
    }

    fn find_single_possible_solutions_for_rows(&mut self) {
        for y in 0..GRID_JOINT_SIZE {
            let row = self.grid.get_horizontal_localized_digits(y);
            Self::get_solutions(row)
                .into_iter()
                .map(|solution| solution.into_owned_tuple())
                .for_each(|(digit, x, y)| self.grid.set_digit(x, y, digit));
        }
    }

    fn get_solutions<'a, 'b>(digits: Vec<LocalizedDigit<'a>>) -> Vec<LocalizedDigit<'b>> {
        let possible_values = Self::get_possible_values(&digits);
        let mut solutions = Vec::with_capacity(9);
        for value in possible_values {
            let mut digits_with_possible_value = Vec::with_capacity(9);
            for digit in &digits {
                if let Digit::Unknown(unknown_digit) = digit.digit.borrow() {
                    let digit_possible_values = &unknown_digit.possible_values;
                    if digit_possible_values.contains(&value) {
                        digits_with_possible_value.push(digit)
                    }
                }
            }
            if digits_with_possible_value.len() == 1 {
                let digit = digits_with_possible_value.swap_remove(0);
                let LocalizedDigit { digit: _, x, y } = digit;
                let digit = Digit::Known(value);
                let digit = LocalizedDigit::from_owned(digit, *x, *y);
                solutions.push(digit);
            }
        }
        solutions
    }

    fn get_possible_values(digits: &[LocalizedDigit]) -> Vec<u32> {
        let mut possible_values = (1..=SUBGRID_LENGTH as u32).collect::<Vec<_>>();
        for digit in digits {
            if let Digit::Known(digit) = digit.digit.borrow() {
                possible_values.retain(|element| element != digit);
            }
        }
        possible_values
    }

    fn set_single_possible_values(&mut self) {
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
        self.eliminate_impossible_possible_values_in_columns();
        self.eliminate_impossible_possible_values_in_rows();
    }

    fn get_complement(neighbours: Vec<&Digit>) -> Vec<u32> {
        let mut complement = (1..=SUBGRID_LENGTH as u32).collect::<Vec<_>>();
        for neighbour in neighbours {
            if let Digit::Known(neighbour) = neighbour {
                complement.retain(|element| element != neighbour);
            }
        }
        complement
    }

    fn eliminate_impossible_possible_values_in_columns(&mut self) {
        for grid_row in 0..GRID_SIZE {
            for column in 0..GRID_JOINT_SIZE {
                let mut column_values = Vec::with_capacity(9);
                for subgrid_y in SUBGRID_SIZE * grid_row..SUBGRID_SIZE * (grid_row + 1) {
                    let digit = self.grid.get_digit(column, subgrid_y);
                    if let Digit::Unknown(digit) = digit {
                        let mut possible_values = digit.possible_values.clone();
                        column_values.append(&mut possible_values)
                    }
                }
                let subgrid = self.grid.get_subgrid(column, SUBGRID_SIZE * grid_row);
                let mut positions = Vec::with_capacity(SUBGRID_SIZE);
                let x = column % SUBGRID_SIZE;
                for y in 0..SUBGRID_SIZE {
                    let position = Position::new(x, y);
                    positions.push(position)
                }
                let mut values_to_remove = Vec::with_capacity(SUBGRID_LENGTH);
                let digits = subgrid.get_digits_excluding(positions);
                for column_value in column_values {
                    if !Self::is_in_digits(column_value, &digits) {
                        values_to_remove.push(column_value);
                    }
                }
                for value in values_to_remove {
                    for row in 0..SUBGRID_SIZE * grid_row {
                        let digit = self.grid.get_digit_mut(column, row);
                        if let Digit::Unknown(digit) = digit {
                            digit.possible_values.retain(|element| *element != value);
                        }
                    }
                    for row in SUBGRID_SIZE * (grid_row + 1)..GRID_JOINT_SIZE {
                        let digit = self.grid.get_digit_mut(column, row);
                        if let Digit::Unknown(digit) = digit {
                            digit.possible_values.retain(|element| *element != value);
                        }
                    }
                }
            }
        }
    }

    fn eliminate_impossible_possible_values_in_rows(&mut self) {
        for grid_column in 0..GRID_SIZE {
            for row in 0..GRID_JOINT_SIZE {
                let mut row_values = Vec::with_capacity(9);
                for subgrid_x in SUBGRID_SIZE * grid_column..SUBGRID_SIZE * (grid_column + 1) {
                    let digit = self.grid.get_digit(subgrid_x, row);
                    if let Digit::Unknown(digit) = digit {
                        let mut possible_values = digit.possible_values.clone();
                        row_values.append(&mut possible_values)
                    }
                }
                let subgrid = self.grid.get_subgrid(SUBGRID_SIZE * grid_column, row);
                let mut positions = Vec::with_capacity(SUBGRID_SIZE);
                let y = row % SUBGRID_SIZE;
                for x in 0..SUBGRID_SIZE {
                    let position = Position::new(x, y);
                    positions.push(position)
                }
                let mut values_to_remove = Vec::with_capacity(SUBGRID_LENGTH);
                let digits = subgrid.get_digits_excluding(positions);
                for row_value in row_values {
                    if !Self::is_in_digits(row_value, &digits) {
                        values_to_remove.push(row_value);
                    }
                }
                for value in values_to_remove {
                    for column in 0..SUBGRID_SIZE * grid_column {
                        let digit = self.grid.get_digit_mut(column, row);
                        if let Digit::Unknown(digit) = digit {
                            digit.possible_values.retain(|element| *element != value);
                        }
                    }
                    for column in SUBGRID_SIZE * (grid_column + 1)..GRID_JOINT_SIZE {
                        let digit = self.grid.get_digit_mut(column, row);
                        if let Digit::Unknown(digit) = digit {
                            digit.possible_values.retain(|element| *element != value);
                        }
                    }
                }
            }
        }
    }

    fn is_in_digits(value: u32, digits: &[&Digit]) -> bool {
        for digit in digits {
            if let Digit::Unknown(digit) = digit {
                if digit.possible_values.contains(&value) {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests;
