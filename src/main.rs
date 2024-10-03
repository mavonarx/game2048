mod tile;
mod game;
mod gameboard;

fn main() {
    let game =  game::Game::new();
    game.print();
}
