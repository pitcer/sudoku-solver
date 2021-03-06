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
use sudoku_solver::grid::parser::Parser;
use sudoku_solver::solver::Solver;

type TestResult = Result<(), TestError>;
type TestError = Cow<'static, str>;

#[test]
fn test_easy_board_is_solved() -> TestResult {
    let grid_syntax = include_str!("grids/easy.grid");
    test_grid_is_solved_correctly(grid_syntax)
}

#[test]
fn test_medium_grid_is_solved_correctly() -> TestResult {
    let grid_syntax = include_str!("grids/medium.grid");
    test_grid_is_solved_correctly(grid_syntax)
}

#[test]
fn test_hard_grid_is_solved_correctly() -> TestResult {
    let grid_syntax = include_str!("grids/hard.grid");
    test_grid_is_solved_correctly(grid_syntax)
}

#[test]
#[ignore]
fn test_extreme_grid_is_solved_correctly() -> TestResult {
    let grid_syntax = include_str!("grids/extreme.grid");
    test_grid_is_solved_correctly(grid_syntax)
}

fn test_grid_is_solved_correctly(board_syntax: &'static str) -> TestResult {
    let grid_syntax = board_syntax.to_owned();
    let parser = Parser::default();
    let grid = parser.parse(grid_syntax)?;
    let solver = Solver::new(grid);
    let solved_grid = solver.solve()?;
    assert!(solved_grid.is_solved_correctly());
    Ok(())
}
