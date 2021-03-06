use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::mem;
use std::hash::Hash;

use super::state;
use super::Solution;
use super::Expandable;

pub fn solve<T>(puzzle: &T, target: &T) -> Option<Solution<T>>
    where T: Expandable + Clone + Eq + Hash {
    // todo: use unsafe code instead of reference counting
    let start = Rc::new(RefCell::new(state::new(puzzle.clone(), 0)));

    // open_rank stores the open states sorted by ranking
    let mut open_rank = Vec::new();
    let mut states = HashMap::new();

    let mut complexity_time = 0;

    open_rank.push(Rc::clone(&start));
    // todo: Duplicating data here.

    let key = start.borrow().data.clone();
    states.insert(key, start);

    loop {
        complexity_time += 1;

        let e_cell = if let Some(state) = open_rank.pop() {
            state
        } else {
            return None;
        };

        let e = e_cell.borrow();

        if e.data == *target {
            let sequence_of_states: Vec<T> = {

                let mut sequence_of_states = vec![];
                sequence_of_states.push(e.data.clone());
                let mut state = mem::replace(&mut e_cell.borrow_mut().previous, None);
                loop {
                    state = match state {
                        Some(s) => {
                            let mut s = s.borrow_mut();
                            sequence_of_states.push(s.data.clone());
                            mem::replace(&mut s.previous, None)
                        },
                        None => break,
                }
                    }
                let sequence_of_states = sequence_of_states.drain(..).rev().collect();
                sequence_of_states
            };

            for state in states.values() {
                if state.borrow().data != e.data {
                    state.borrow_mut().previous = None;
                }
            }

            return Some(Solution {
                complexity_time,
                complexity_space: states.len(),
                sequence_of_states,
                number_of_moves_required: e.distance,
            });
        };

        for mut s in e.expand() {
            // Two if statements so I can mutate states in else.
            if ( if let Some(mut s_cell) = states.get(&s.data) {
                let mut s = s_cell.borrow_mut();
                if s.distance > e.distance + 1 {
                    s.distance = e.distance + 1;
                    s.previous = Some(Rc::clone(&e_cell));
                    if s.open == false {
                        s.open = true;
                        open_rank.push(Rc::clone(&s_cell));
                    }
                }
                true
            } else { false }) == false {

                s.previous = Some(Rc::clone(&e_cell));
                let s = Rc::new(RefCell::new(s));
                open_rank.push(Rc::clone(&s));
                let key = s.borrow().data.clone();
                states.insert(key, s);
            }
        }
    }
}
