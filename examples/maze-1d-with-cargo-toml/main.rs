
use my_rust_lib::maze_1d::*;

fn main() {
    let start_pos = 2;
    let start_dir = Dir::L;
    goto(&start_dir, start_pos, &mut steps, &mut all_steps);
}