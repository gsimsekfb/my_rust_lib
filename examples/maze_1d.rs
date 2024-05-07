
#[path = "../src/maze_1d.rs"] mod maze_1d;
use maze_1d::*;

fn main() {
    let start_pos = 2;
    let start_dir = Dir::L;
    start_game(&start_dir, start_pos);
}