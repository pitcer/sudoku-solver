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

use std::io;
use std::io::{Error, Read};

use sudoku_solver::grid::generator::Generator;
use sudoku_solver::grid::parser::Parser;
use sudoku_solver::solver::Solver;

fn main() {
    let mut stdin = io::stdin();
    let grid_syntax = read_input(&mut stdin).expect("Cannot read from std input");
    let parser = Parser::default();
    let grid = parser.parse(grid_syntax).expect("Cannot parse grid syntax");
    let solver = Solver::new(grid);
    let solved_grid = solver.solve().expect("Cannot solve the given grid");
    let generator = Generator::default();
    let readable_grid = generator
        .generate(&solved_grid)
        .expect("Cannot generate readable grid");
    println!("{}", readable_grid);
    if solved_grid.is_solved_correctly() {
        println!("Grid has been solved correctly.");
    } else {
        println!("Grid has been solved incorrectly!");
    }
}

fn read_input(stdin: &mut dyn Read) -> Result<String, Error> {
    let mut result = String::new();
    stdin.read_to_string(&mut result)?;
    Ok(result)
}
