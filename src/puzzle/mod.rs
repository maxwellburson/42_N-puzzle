use std::fmt;

mod parse;
mod solve;

pub struct Puzzle {
    size: usize,
    content: Vec<Vec<i32>>,
}

pub struct Solution {
    // Total number of states ever selected in the "opened" set (complexity in time).
    complexity_time: u32,
    // Maximum number of states ever represented in memory at the same time
    // during the search (complexity in size)
    complexity_space: u32,
    // Number of moves required to transition from the initial state to the final state,
    // according to the search.
    moves: (), // unimplemented
}
   
pub fn new(raw: &str) -> Result<Puzzle, String> {
    parse::parse_puzzle(raw)
}

trait Solve {
    fn solve(&self) -> Result<Solution, ()>;
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{:?}", self.content)
    }
}

impl Solve for Puzzle {
    fn solve(&self) -> Result<Solution, ()> {
        solve::solve(self)
    }
}