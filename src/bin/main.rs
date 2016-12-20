extern crate rustqueens;

fn main() {
    let initial_board = rustqueens::Board::empty_board(12);
    for board in initial_board.solve() {
        board.pprint();
    }
}
