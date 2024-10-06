use crate::gameboard::GameBoard;

pub struct Game {
    is_won: bool,
    game_board: GameBoard,
}

impl Game {
    pub fn new() -> Self {
        Self {
            is_won: false,
            game_board: GameBoard::new(),
        }
    }

    pub fn print(&self) {
        self.game_board.print();
    }
}
