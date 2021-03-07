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

use crate::grid::digit::{Digit, LocalizedDigit};
use crate::grid::position::Position;

pub mod digit;
pub mod generator;
pub mod parser;
pub mod position;

pub const GRID_SIZE: usize = 3;
pub const GRID_LENGTH: usize = GRID_SIZE * GRID_SIZE;
pub const GRID_JOINT_SIZE: usize = GRID_SIZE * SUBGRID_SIZE;

pub const SUBGRID_SIZE: usize = 3;
pub const SUBGRID_LENGTH: usize = SUBGRID_SIZE * SUBGRID_SIZE;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Grid {
    pub subgrids: Vec<Subgrid>,
}

impl Default for Grid {
    fn default() -> Self {
        let subgrids = vec![Subgrid::default(); GRID_LENGTH];
        Self::new(subgrids)
    }
}

impl Grid {
    pub fn new(subgrids: Vec<Subgrid>) -> Self {
        Self { subgrids }
    }

    pub fn is_solved(&self) -> bool {
        for subgrid in &self.subgrids {
            if !subgrid.is_solved() {
                return false;
            }
        }
        true
    }

    pub fn is_solved_correctly(&self) -> bool {
        for x in 0..GRID_JOINT_SIZE {
            for y in 0..GRID_JOINT_SIZE {
                let digit = self.get_digit(x, y);
                let neighbours = self.get_neighbour_digits(x, y);
                if neighbours.contains(&digit) {
                    return false;
                }
            }
        }
        true
    }

    pub fn get_neighbour_digits(&self, x_joint: usize, y_joint: usize) -> Vec<&Digit> {
        let subgrid = self.get_subgrid(x_joint, y_joint);
        let (subgrid_x, subgrid_y) = to_subgrid_coordinates(x_joint, y_joint);
        let mut neighbours = subgrid.get_neighbour_digits(subgrid_x, subgrid_y);
        let (grid_x, grid_y) = to_grid_coordinates(x_joint, y_joint);
        let vertical_skip = grid_y * SUBGRID_SIZE;
        let mut vertical_upper = (0..vertical_skip)
            .map(|y_joint| self.get_digit(x_joint, y_joint))
            .collect::<Vec<_>>();
        let mut vertical_lower = (vertical_skip + SUBGRID_SIZE..GRID_JOINT_SIZE)
            .map(|y_joint| self.get_digit(x_joint, y_joint))
            .collect::<Vec<_>>();
        let horizontal_skip = grid_x * SUBGRID_SIZE;
        let mut horizontal_upper = (0..horizontal_skip)
            .map(|x_joint| self.get_digit(x_joint, y_joint))
            .collect::<Vec<_>>();
        let mut horizontal_lower = (horizontal_skip + SUBGRID_SIZE..GRID_JOINT_SIZE)
            .map(|x_joint| self.get_digit(x_joint, y_joint))
            .collect::<Vec<_>>();
        neighbours.append(&mut vertical_upper);
        neighbours.append(&mut vertical_lower);
        neighbours.append(&mut horizontal_upper);
        neighbours.append(&mut horizontal_lower);
        neighbours
    }

    pub fn get_vertical_digits(&self, x_joint: usize) -> Vec<&Digit> {
        (0..GRID_JOINT_SIZE)
            .map(|y_joint| self.get_digit(x_joint, y_joint))
            .collect()
    }

    pub fn get_vertical_localized_digits(&self, x_joint: usize) -> Vec<LocalizedDigit> {
        (0..GRID_JOINT_SIZE)
            .map(|y_joint| {
                let digit = self.get_digit(x_joint, y_joint);
                LocalizedDigit::from_borrowed(digit, x_joint, y_joint)
            })
            .collect()
    }

    pub fn get_horizontal_digits(&self, y_joint: usize) -> Vec<&Digit> {
        (0..GRID_JOINT_SIZE)
            .map(|x_joint| self.get_digit(x_joint, y_joint))
            .collect()
    }

    pub fn get_horizontal_localized_digits(&self, y_joint: usize) -> Vec<LocalizedDigit> {
        (0..GRID_JOINT_SIZE)
            .map(|x_joint| {
                let digit = self.get_digit(x_joint, y_joint);
                LocalizedDigit::from_borrowed(digit, x_joint, y_joint)
            })
            .collect()
    }

    pub fn get_digit(&self, x_joint: usize, y_joint: usize) -> &Digit {
        let subgrid = self.get_subgrid(x_joint, y_joint);
        let (subgrid_x, subgrid_y) = to_subgrid_coordinates(x_joint, y_joint);
        subgrid.get_digit(subgrid_x, subgrid_y)
    }

    pub fn get_digit_mut(&mut self, x_joint: usize, y_joint: usize) -> &mut Digit {
        let subgrid = self.get_subgrid_mut(x_joint, y_joint);
        let (subgrid_x, subgrid_y) = to_subgrid_coordinates(x_joint, y_joint);
        subgrid.get_digit_mut(subgrid_x, subgrid_y)
    }

    pub fn set_digit(&mut self, x_joint: usize, y_joint: usize, digit: Digit) {
        let subgrid = self.get_subgrid_mut(x_joint, y_joint);
        let (subgrid_x, subgrid_y) = to_subgrid_coordinates(x_joint, y_joint);
        subgrid.set_digit(subgrid_x, subgrid_y, digit);
    }

    pub fn get_subgrid(&self, x_joint: usize, y_joint: usize) -> &Subgrid {
        let (grid_x, grid_y) = to_grid_coordinates(x_joint, y_joint);
        self.get_subgrid_absolute(grid_x, grid_y)
    }

    pub fn get_subgrid_mut(&mut self, x_joint: usize, y_joint: usize) -> &mut Subgrid {
        let (grid_x, grid_y) = to_grid_coordinates(x_joint, y_joint);
        self.get_subgrid_absolute_mut(grid_x, grid_y)
    }

    pub fn get_subgrid_absolute(&self, x: usize, y: usize) -> &Subgrid {
        validate_x(x, GRID_SIZE);
        validate_y(y, GRID_SIZE);
        let index = self.to_grid_index(x, y);
        self.subgrids.get(index).unwrap()
    }

    pub fn get_subgrid_absolute_mut(&mut self, x: usize, y: usize) -> &mut Subgrid {
        validate_x(x, GRID_SIZE);
        validate_y(y, GRID_SIZE);
        let index = self.to_grid_index(x, y);
        self.subgrids.get_mut(index).unwrap()
    }

    fn to_grid_index(&self, x: usize, y: usize) -> usize {
        x + y * GRID_SIZE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Subgrid {
    pub digits: Vec<Digit>,
}

impl Default for Subgrid {
    fn default() -> Self {
        let digits = vec![Digit::default(); SUBGRID_LENGTH];
        Self::new(digits)
    }
}

impl Subgrid {
    pub fn new(digits: Vec<Digit>) -> Self {
        Self { digits }
    }

    #[cfg(test)]
    pub fn from_digits(digits: Vec<u32>) -> Self {
        let digits = digits.into_iter().map(Digit::from).collect();
        Self::new(digits)
    }

    pub fn is_solved(&self) -> bool {
        for digit in &self.digits {
            if let Digit::Unknown(_) = digit {
                return false;
            }
        }
        true
    }

    pub fn get_digits_excluding(&self, positions: Vec<Position>) -> Vec<&Digit> {
        let mut digits = Vec::with_capacity(SUBGRID_LENGTH);
        for index in 0..SUBGRID_LENGTH {
            let position = Position::from_index(index);
            if !positions.contains(&position) {
                let digit = self.digits.get(index).unwrap();
                digits.push(digit);
            }
        }
        digits
    }

    pub fn get_neighbour_digits(&self, x: usize, y: usize) -> Vec<&Digit> {
        validate_x(x, SUBGRID_SIZE);
        validate_y(y, SUBGRID_SIZE);
        let mut neighbours = self.digits();
        let index = self.to_subgrid_index(x, y);
        neighbours.swap_remove(index);
        neighbours
    }

    pub fn get_digit(&self, x: usize, y: usize) -> &Digit {
        validate_x(x, SUBGRID_SIZE);
        validate_y(y, SUBGRID_SIZE);
        let index = self.to_subgrid_index(x, y);
        self.digits.get(index).unwrap()
    }

    pub fn get_digit_mut(&mut self, x: usize, y: usize) -> &mut Digit {
        validate_x(x, SUBGRID_SIZE);
        validate_y(y, SUBGRID_SIZE);
        let index = self.to_subgrid_index(x, y);
        self.digits.get_mut(index).unwrap()
    }

    pub fn set_digit(&mut self, x: usize, y: usize, digit: Digit) {
        validate_x(x, SUBGRID_SIZE);
        validate_y(y, SUBGRID_SIZE);
        let index = self.to_subgrid_index(x, y);
        self.digits[index] = digit;
    }

    pub fn digits(&self) -> Vec<&Digit> {
        self.digits.iter().collect()
    }

    pub fn localized_digits(&self) -> Vec<LocalizedDigit> {
        self.digits
            .iter()
            .enumerate()
            .map(|(index, digit)| {
                let (x, y) = self.to_coordinates(index);
                LocalizedDigit::from_borrowed(digit, x, y)
            })
            .collect()
    }

    fn to_subgrid_index(&self, x: usize, y: usize) -> usize {
        x + y * SUBGRID_SIZE
    }

    fn to_coordinates(&self, index: usize) -> (usize, usize) {
        (index % SUBGRID_SIZE, index / SUBGRID_SIZE)
    }
}

fn validate_x(x: usize, max_size: usize) {
    assert!(
        (0..max_size).contains(&x),
        "x should be between 0 and {}",
        max_size - 1
    );
}

fn validate_y(y: usize, max_size: usize) {
    assert!(
        (0..max_size).contains(&y),
        "y should be between 0 and {}",
        max_size - 1
    );
}

fn to_grid_coordinates(x: usize, y: usize) -> (usize, usize) {
    (x / SUBGRID_SIZE, y / SUBGRID_SIZE)
}

fn to_subgrid_coordinates(x: usize, y: usize) -> (usize, usize) {
    (x % GRID_SIZE, y % GRID_SIZE)
}

#[cfg(test)]
mod tests {
    use crate::grid::digit::UnknownDigit;

    use super::*;

    #[test]
    fn test_grid_get_digit() {
        let grid = create_grid();
        assert_eq!(grid.get_digit(0, 0), &Digit::Known(1));
        assert_eq!(grid.get_digit(2, 2), &Digit::Known(9));
        assert_eq!(grid.get_digit(8, 2), &Digit::Known(9));
        assert_eq!(grid.get_digit(4, 4), &Digit::Known(5));
        assert_eq!(grid.get_digit(7, 8), &Digit::Known(7));
    }

    #[test]
    fn test_grid_get_vertical_digits() {
        let grid = create_grid();
        assert_eq!(
            grid.get_vertical_digits(3),
            vec![
                &Digit::Known(9),
                &Digit::Known(6),
                &Digit::Known(3),
                &Digit::Known(1),
                &Digit::Known(4),
                &Digit::Known(7),
                &Digit::Unknown(UnknownDigit::default()),
                &Digit::Unknown(UnknownDigit::default()),
                &Digit::Unknown(UnknownDigit::default())
            ]
        );
    }

    #[test]
    fn test_grid_get_vertical_localized_digits() {
        let grid = create_grid();
        assert_eq!(
            grid.get_vertical_localized_digits(3),
            vec![
                LocalizedDigit::from_owned(Digit::Known(9), 3, 0),
                LocalizedDigit::from_owned(Digit::Known(6), 3, 1),
                LocalizedDigit::from_owned(Digit::Known(3), 3, 2),
                LocalizedDigit::from_owned(Digit::Known(1), 3, 3),
                LocalizedDigit::from_owned(Digit::Known(4), 3, 4),
                LocalizedDigit::from_owned(Digit::Known(7), 3, 5),
                LocalizedDigit::from_owned(Digit::Unknown(UnknownDigit::default()), 3, 6),
                LocalizedDigit::from_owned(Digit::Unknown(UnknownDigit::default()), 3, 7),
                LocalizedDigit::from_owned(Digit::Unknown(UnknownDigit::default()), 3, 8),
            ]
        );
    }

    #[test]
    fn test_grid_get_horizontal_digits() {
        let grid = create_grid();
        assert_eq!(
            grid.get_horizontal_digits(5),
            vec![
                &Digit::Known(3),
                &Digit::Known(2),
                &Digit::Known(1),
                &Digit::Known(7),
                &Digit::Known(8),
                &Digit::Known(9),
                &Digit::Unknown(UnknownDigit::default()),
                &Digit::Unknown(UnknownDigit::default()),
                &Digit::Known(1)
            ]
        );
    }

    #[test]
    fn test_grid_get_horizontal_localized_digits() {
        let grid = create_grid();
        assert_eq!(
            grid.get_horizontal_localized_digits(5),
            vec![
                LocalizedDigit::from_owned(Digit::Known(3), 0, 5),
                LocalizedDigit::from_owned(Digit::Known(2), 1, 5),
                LocalizedDigit::from_owned(Digit::Known(1), 2, 5),
                LocalizedDigit::from_owned(Digit::Known(7), 3, 5),
                LocalizedDigit::from_owned(Digit::Known(8), 4, 5),
                LocalizedDigit::from_owned(Digit::Known(9), 5, 5),
                LocalizedDigit::from_owned(Digit::Unknown(UnknownDigit::default()), 6, 5),
                LocalizedDigit::from_owned(Digit::Unknown(UnknownDigit::default()), 7, 5),
                LocalizedDigit::from_owned(Digit::Known(1), 8, 5),
            ]
        );
    }

    #[test]
    fn test_grid_get_neighbour_digits() {
        let grid = create_grid();
        let mut actual = grid.get_neighbour_digits(5, 3);
        actual.sort_unstable();
        let unknown_digit = Digit::Unknown(UnknownDigit::default());
        let mut expected = vec![
            &Digit::Known(1),
            &Digit::Known(2),
            &Digit::Known(4),
            &Digit::Known(5),
            &Digit::Known(6),
            &Digit::Known(7),
            &Digit::Known(8),
            &Digit::Known(9),
            &Digit::Known(7),
            &Digit::Known(4),
            &Digit::Known(1),
            &unknown_digit,
            &unknown_digit,
            &unknown_digit,
            &Digit::Known(9),
            &Digit::Known(8),
            &Digit::Known(7),
            &Digit::Known(9),
            &unknown_digit,
            &unknown_digit,
        ];
        expected.sort_unstable();
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_grid_set_digit() {
        let mut grid = create_grid();
        let expected = Grid::new(vec![
            Subgrid::from_digits(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
            Subgrid::from_digits(vec![9, 8, 7, 6, 5, 4, 3, 2, 1]),
            Subgrid::from_digits(vec![1, 0, 0, 0, 0, 0, 0, 0, 9]),
            Subgrid::from_digits(vec![9, 8, 7, 6, 5, 4, 3, 2, 1]),
            Subgrid::from_digits(vec![1, 2, 9, 4, 5, 6, 7, 8, 9]),
            Subgrid::from_digits(vec![9, 0, 0, 0, 0, 0, 0, 0, 1]),
            Subgrid::default(),
            Subgrid::default(),
            Subgrid::from_digits(vec![0, 0, 0, 0, 0, 0, 0, 7, 0]),
        ]);
        grid.set_digit(5, 3, Digit::Known(9));
        assert_eq!(expected, grid);
    }

    #[test]
    fn test_subgrid_localized_digits() {
        let subgrid = Subgrid::from_digits(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let expected = vec![
            LocalizedDigit::from_owned(Digit::Known(1), 0, 0),
            LocalizedDigit::from_owned(Digit::Known(2), 1, 0),
            LocalizedDigit::from_owned(Digit::Known(3), 2, 0),
            LocalizedDigit::from_owned(Digit::Known(4), 0, 1),
            LocalizedDigit::from_owned(Digit::Known(5), 1, 1),
            LocalizedDigit::from_owned(Digit::Known(6), 2, 1),
            LocalizedDigit::from_owned(Digit::Known(7), 0, 2),
            LocalizedDigit::from_owned(Digit::Known(8), 1, 2),
            LocalizedDigit::from_owned(Digit::Known(9), 2, 2),
        ];
        let actual = subgrid.localized_digits();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_subgrid_get_digits_excluding() {
        let subgrid = Subgrid::from_digits(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let expected = vec![
            &Digit::Known(1),
            &Digit::Known(3),
            &Digit::Known(4),
            &Digit::Known(5),
            &Digit::Known(7),
            &Digit::Known(8),
        ];
        let actual = subgrid.get_digits_excluding(vec![
            Position::new(1, 0),
            Position::new(2, 1),
            Position::new(2, 2),
        ]);
        assert_eq!(expected, actual);
    }

    fn create_grid() -> Grid {
        Grid::new(vec![
            Subgrid::from_digits(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
            Subgrid::from_digits(vec![9, 8, 7, 6, 5, 4, 3, 2, 1]),
            Subgrid::from_digits(vec![1, 0, 0, 0, 0, 0, 0, 0, 9]),
            Subgrid::from_digits(vec![9, 8, 7, 6, 5, 4, 3, 2, 1]),
            Subgrid::from_digits(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
            Subgrid::from_digits(vec![9, 0, 0, 0, 0, 0, 0, 0, 1]),
            Subgrid::default(),
            Subgrid::default(),
            Subgrid::from_digits(vec![0, 0, 0, 0, 0, 0, 0, 7, 0]),
        ])
    }
}
