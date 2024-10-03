use crate::tile::Tile;
pub struct GameBoard {
    game_board: [[Tile;4];4]
}

impl GameBoard {
    pub fn new () -> Self {
        Self {
            game_board: [[Tile::new(0);4];4],
        }
        
    }

    pub fn print(&self) {
        for i in 0..4 {
            println!(" _  _  _  _");
            
            for j in 0..4 {
                print!("|{} ", self.game_board[i][j])
            }
            println!("|");
            
        }
        println!(" _  _  _  _");
    }
}