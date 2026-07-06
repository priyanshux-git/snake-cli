mod game;
fn main(){
    let mut game = game::Game::init();
    game.start();
}