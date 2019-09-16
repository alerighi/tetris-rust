// TODO:
//  - next piece preview
//  - high score
//  - use an async mechanism to advance pieces

mod game;
mod ui; 

fn main() {
    ui::Ui::new().game_loop();
}
