use crate::gameboard::GameBoard;

pub struct Game {
    is_won: bool,
    game_board: GameBoard,
    
}

impl Game {
    pub fn new () -> Self {
        let board = Self {
            is_won: false,
            game_board: GameBoard::new(),
        };
        

        board
    }

    pub fn print(&self)  {
        self.game_board.print();
    }

}