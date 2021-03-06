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
use std::str::Chars;

use crate::grid::digit::Digit;
use crate::grid::{Grid, Subgrid, GRID_JOINT_SIZE, GRID_LENGTH, GRID_SIZE, SUBGRID_SIZE};

pub type ParserResult = Result<Grid, ParserError>;
pub type ParserError = Cow<'static, str>;

pub struct Parser;

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&self, syntax: String) -> ParserResult {
        let characters = syntax.chars();
        let subgrids_digits = self.parse_characters(characters)?;
        let subgrids = subgrids_digits
            .into_iter()
            .map(Subgrid::new)
            .collect::<Vec<Subgrid>>();
        let grid = Grid::new(subgrids);
        Ok(grid)
    }

    fn parse_characters(&self, characters: Chars) -> Result<Vec<Vec<Digit>>, ParserError> {
        let mut digit_counter = 0;
        let mut subgrids_digits: Vec<Vec<_>> = vec![Vec::default(); GRID_LENGTH];
        for character in characters {
            self.parse_character(character, &mut digit_counter, &mut subgrids_digits)?;
        }
        Ok(subgrids_digits)
    }

    fn parse_character(
        &self,
        character: char,
        digit_counter: &mut usize,
        subgrids_digits: &mut Vec<Vec<Digit>>,
    ) -> Result<(), ParserError> {
        match character {
            '0'..='9' => self.parse_digit(character, digit_counter, subgrids_digits),
            ',' | ';' | '\n' | ' ' => Ok(()),
            _ => Err(format!("Invalid character: '{}'", character).into()),
        }
    }

    fn parse_digit(
        &self,
        character: char,
        digit_counter: &mut usize,
        subgrids_digits: &mut Vec<Vec<Digit>>,
    ) -> Result<(), ParserError> {
        let counter = *digit_counter;
        let row = counter / (GRID_JOINT_SIZE * GRID_SIZE);
        let index = GRID_SIZE * row + (counter / SUBGRID_SIZE) % GRID_SIZE;
        let digits = subgrids_digits.get_mut(index).ok_or("Invalid index")?;
        let digit = character.to_digit(10).ok_or("Invalid character")?;
        let digit = Digit::from(digit);
        digits.push(digit);
        *digit_counter += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestResult = Result<(), ParserError>;

    #[test]
    fn test_grid_is_parsed_correctly() -> TestResult {
        let expected = Grid::new(vec![
            Subgrid::from_digits(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
            Subgrid::from_digits(vec![9, 8, 7, 6, 5, 4, 3, 2, 1]),
            Subgrid::from_digits(vec![1, 0, 0, 0, 0, 0, 0, 0, 9]),
            Subgrid::from_digits(vec![9, 8, 7, 6, 5, 4, 3, 2, 1]),
            Subgrid::from_digits(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
            Subgrid::from_digits(vec![9, 0, 0, 0, 0, 0, 0, 0, 1]),
            Subgrid::default(),
            Subgrid::default(),
            Subgrid::default(),
        ]);
        let parser = Parser::default();
        let syntax = include_str!("test.grid").to_owned();
        let actual = parser.parse(syntax)?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
