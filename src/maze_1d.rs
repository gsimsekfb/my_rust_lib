
#[derive(Clone, Debug, PartialEq)]
pub enum Dir { L, R }  // Left, Right

// 1D world
// 1 is open, 0 is closed box
const W: [usize; 5] = [1, 0, 0, 1, 1];

pub fn start_game(
    dir: &Dir,
    pos: usize,
) {
    print_info(dir, pos);

    let mut all_steps: Vec<Dir> = vec![];
    let mut steps: Vec<Dir> = vec![];    
    goto(dir, pos, &mut steps, &mut all_steps);

    println!("\n##### Game ends: You win! #####");
    println!("steps:     {:?}", steps);
    println!("all_steps: {:?}\n", all_steps);
}

pub fn print_info(start_dir: &Dir, start_pos: usize) {
    println!("");
    println!("--INFO-------------------------");
    println!("World: {:?}", W);
    println!("Start pos: {start_pos}");
    println!("Start dir: {:?}", start_dir);
    println!("starting game...");
    println!("-------------------------------");
}

pub fn goto(
    dir: &Dir,
    pos: usize,
    steps: &mut Vec<Dir>,
    all_steps: &mut Vec<Dir>
) {
    // println!("-- going from {pos}");
    if pos == 0 { return }
    if pos == W.len()-1 { return }
    match dir {
        Dir::L => {
            all_steps.push(dir.clone());
            if W[pos-1] == 1 {  // Open cell
                // println!("L");
                steps.push(dir.clone());
                goto(dir, pos-1, steps, all_steps);
            } else {
                if W[pos+1] == 0 { // Closed cell
                    panic!("All ways are closed. pos: {pos}");
                } else {
                    goto(&Dir::R, pos, steps, all_steps);
                }
            }
        }
        Dir::R => {
            all_steps.push(dir.clone());
            if W[pos+1] == 1 {
                // println!("R");
                steps.push(dir.clone());
                goto(dir, pos+1, steps, all_steps)
            } else {
                if W[pos-1] == 0 {
                    panic!("All ways are closed. pos: {pos}");
                } else {                
                    goto(&Dir::L, pos, steps, all_steps);
                }
            }
        }
    };
}
