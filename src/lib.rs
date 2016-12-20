#[derive(Clone, Debug)]
pub struct Board {
    positions: Vec<Option<usize>>,
}

impl Board {
    pub fn empty_board(size: usize) -> Board {
        Board { positions: vec!(None; size) }
    }

    fn size(&self) -> usize {
        self.positions.len()
    }

    pub fn pprint(&self) {
        for y in 0..self.size() {
            for x in 0..self.size() {
                if self.positions[x].is_some() {
                    if self.positions[x].unwrap() == y {
                        print!("Q");
                        continue;
                    }
                }
                if self.valid(x, y) {
                    print!(" ");
                } else {
                    print!("X");
                }
            }
            println!("");
        }
        println!("");
    }


    pub fn put(&self, x: usize, y: usize) -> Board {
        let mut new_positions: Vec<Option<usize>> = vec!(None; self.size());
        for n in 0..self.size() {
            new_positions[n] = if n == x { Some(y) } else { self.positions[n] };
        }
        Board { positions: new_positions }
    }

    /// ```
    /// use rustqueens::Board;
    ///
    /// let board = Board::empty_board(8);
    /// assert_eq!(true, board.valid(0, 0));
    /// let new_board = board.put(1,1);
    /// assert_eq!(false, new_board.valid(1, 1));
    /// assert_eq!(false, new_board.valid(2, 1));
    /// assert_eq!(false, new_board.valid(1, 2));
    /// assert_eq!(false, new_board.valid(0, 0));
    /// assert_eq!(false, new_board.valid(0, 2));
    /// ```
    pub fn valid(&self, x: usize, y: usize) -> bool {
        if let Some(y_) = self.positions[x] {
            if y_ == y {
                return false;
            }
        }
        for x_ in 0..self.size() {
            if let Some(y_) = self.positions[x_] {
                if x_ == x {
                    return false;
                }
                if y_ == y {
                    return false;
                }
                if (x as isize - y as isize) == (x_ as isize - y_ as isize) {
                    return false;
                }
                if x + y == x_ + y_ {
                    return false;
                }
            }
        }
        true
    }

    fn solved(&self) -> bool {
        for n in 0..self.size() {
            if self.positions[n].is_none() {
                return false;
            }
        }
        true
    }

    pub fn solve(self) -> Vec<Board> {
        if self.solved() {
            return vec![self];
        }
        let mut solutions = vec![];
        for n in 0..self.size() {
            if self.positions[n].is_none() {
                for p in 0..self.size() {
                    if self.valid(n, p) {
                        let possible = self.put(n, p);
                        solutions.append(&mut possible.solve());
                    }
                }
                break;
            }
        }
        return solutions;
    }
}
