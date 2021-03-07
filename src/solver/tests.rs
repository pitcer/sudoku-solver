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

use crate::grid::digit::UnknownDigit;
use crate::grid::Subgrid;

use super::*;

#[test]
fn test_eliminate_impossible_possible_values_in_columns() {
    let grid = Grid::new(vec![
        Subgrid::new(vec![
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
        ]),
        Subgrid::default(),
        Subgrid::default(),
        Subgrid::new(vec![
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
        ]),
        Subgrid::default(),
        Subgrid::default(),
        Subgrid::new(vec![
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
        ]),
        Subgrid::default(),
        Subgrid::default(),
    ]);
    let expected = Grid::new(vec![
        Subgrid::new(vec![
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
        ]),
        Subgrid::default(),
        Subgrid::default(),
        Subgrid::new(vec![
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
        ]),
        Subgrid::default(),
        Subgrid::default(),
        Subgrid::new(vec![
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
        ]),
        Subgrid::default(),
        Subgrid::default(),
    ]);
    let mut solver = Solver::new(grid);
    solver.eliminate_impossible_possible_values_in_columns();
    assert_eq!(expected, solver.grid);
}

#[test]
fn test_eliminate_impossible_possible_values_in_columns_full() {
    let grid = Grid::new(vec![
        Subgrid::new(vec![
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
        ]),
        Subgrid::default(),
        Subgrid::default(),
        Subgrid::new(vec![
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
        ]),
        Subgrid::default(),
        Subgrid::default(),
        Subgrid::new(vec![
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
        ]),
        Subgrid::default(),
        Subgrid::default(),
    ]);
    let expected = Grid::new(vec![
        Subgrid::new(vec![
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
        ]),
        Subgrid::default(),
        Subgrid::default(),
        Subgrid::new(vec![
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
        ]),
        Subgrid::default(),
        Subgrid::default(),
        Subgrid::new(vec![
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
        ]),
        Subgrid::default(),
        Subgrid::default(),
    ]);
    let mut solver = Solver::new(grid);
    solver.eliminate_impossible_possible_values_in_columns();
    assert_eq!(expected, solver.grid);
}

#[test]
fn test_eliminate_impossible_possible_values_in_rows() {
    let grid = Grid::new(vec![
        Subgrid::new(vec![
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
        ]),
        Subgrid::new(vec![
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
        ]),
        Subgrid::new(vec![
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
        ]),
        Subgrid::default(),
        Subgrid::default(),
        Subgrid::default(),
        Subgrid::default(),
        Subgrid::default(),
        Subgrid::default(),
    ]);
    let expected = Grid::new(vec![
        Subgrid::new(vec![
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
        ]),
        Subgrid::new(vec![
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
        ]),
        Subgrid::new(vec![
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
        ]),
        Subgrid::default(),
        Subgrid::default(),
        Subgrid::default(),
        Subgrid::default(),
        Subgrid::default(),
        Subgrid::default(),
    ]);
    let mut solver = Solver::new(grid);
    solver.eliminate_impossible_possible_values_in_rows();
    assert_eq!(expected, solver.grid);
}

#[test]
fn test_eliminate_impossible_possible_values_in_rows_full() {
    let grid = Grid::new(vec![
        Subgrid::new(vec![
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
        ]),
        Subgrid::new(vec![
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
        ]),
        Subgrid::new(vec![
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
        ]),
        Subgrid::default(),
        Subgrid::default(),
        Subgrid::default(),
        Subgrid::default(),
        Subgrid::default(),
        Subgrid::default(),
    ]);
    let expected = Grid::new(vec![
        Subgrid::new(vec![
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
        ]),
        Subgrid::new(vec![
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
        ]),
        Subgrid::new(vec![
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
            Digit::Unknown(UnknownDigit::new(vec![1, 2])),
        ]),
        Subgrid::default(),
        Subgrid::default(),
        Subgrid::default(),
        Subgrid::default(),
        Subgrid::default(),
        Subgrid::default(),
    ]);
    let mut solver = Solver::new(grid);
    solver.eliminate_impossible_possible_values_in_rows();
    assert_eq!(expected, solver.grid);
}
