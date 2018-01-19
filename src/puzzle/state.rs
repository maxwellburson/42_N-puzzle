use std::cmp::Eq;
use std::cmp::PartialEq;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

#[derive(Clone, Debug)]
pub struct State {
    pub data: Vec<Vec<i32>>,
    x: usize,
    y: usize,
    pub predecessor: Option<Rc<RefCell<State>>>,
    pub distance: usize,
    pub open: bool,
}

pub fn new(data: Vec<Vec<i32>>, distance: usize) -> State {
    let mut coordinate: Option<(usize, usize)> = None;

    for (y, row) in data.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if *value == 0 {
                coordinate = Some((x, y));
            }
        }
    }

    if let Some((x, y)) = coordinate {
        return State {
            data, x, y, distance,
            predecessor: None,
            open: true,
        };
    }

    panic!("Failed to find empty space in puzzle");
}

impl State {
    fn move_space(&self, x: usize, y: usize) -> State {
        let mut data = self.data.clone();

        let swap = data[y][x];
        data[y][x] = data[self.y][self.x];
        data[self.y][self.x] = swap;

        State {
            data, x, y,
            predecessor: None,
            distance: self.distance,
            open: true,
        }
    }
    pub fn expand(&self) -> Vec<State> {
        let size = self.data.len();

        // todo: numbers wont go below zero and get culled

        let mut moves = Vec::new();

        if self.x > 0 {
            moves.push((self.x - 1, self.y));
        }
        if self.x  + 1 < size {
            moves.push((self.x + 1, self.y));
        }
        if self.y > 0 {
            moves.push(((self.x, self.y - 1)));
        }
        if self.y + 1 < size {
            moves.push(((self.x, self.y + 1)));
        }
        moves.iter()
            .map(|m| self.move_space(m.0, m.1))
            .collect()
    }
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.data == other.data
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: & mut fmt::Formatter) -> fmt::Result {
        for line in self.data.iter() {
            for n in line.iter() {
                write!(f, "{} ", n)?;
            }
            write!(f, "\n")?;
        };
        Ok(())
    }
}

#[test]
fn test_expand() {
    let base = Rc::new(RefCell::new(
        State {
            data: vec![
                vec![1, 0],
                vec![1, 1],
            ],
            x: 1,
            y: 0,
            distance: 0,
            open: false,
            predecessor: None
        }
    ));

    assert_eq!(
        vec![
            State {
                data: vec![
                    vec![0, 1],
                    vec![1, 1],
                ],
                x: 0,
                y: 0,
                distance: 1,
                open: true,
                predecessor: Some(Rc::clone(&base)),
            },
            State {
                data: vec![
                    vec![1, 1],
                    vec![1, 0],
                ],
                x: 1,
                y: 1,
                distance: 1,
                open: true,
                predecessor: Some(Rc::clone(&base)),
            }
        ],
        base.borrow().expand()
    );
}
