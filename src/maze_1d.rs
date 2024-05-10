
#[derive(Clone, Debug, PartialEq)]
pub enum Dir { L, R }  // Left, Right

// 1D world
// 1 open, 0 closed, cell [0,1,2,3,4,5,6] 
const WORLD: [usize; 5] = [1,0,0,1,1];
// const WORLD: [usize; 7] = [0,1,1,0,1,1,0]; // todo

// todo: 
// - save R dir is closed on failure

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

pub fn got_stuck(pos: usize) {
    println!("Got stuck! All cells around are closed. pos: {pos}");
    std::process::exit(0);
}

pub fn print_info(start_dir: &Dir, start_pos: usize) {
    println!("\n--INFO-------------------------");
    println!("World: {:?}", WORLD);
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
    if pos == WORLD.len()-1 { return }
    if all_steps.len() == 10 { 
        println!("\n!!! Error: # of steps limit exceeded"); return;
    }
    match dir {
        Dir::L => {
            all_steps.push(dir.clone());
            if WORLD[pos-1] == 1 {  // Open cell
                println!("L");
                steps.push(dir.clone());
                goto(dir, pos-1, steps, all_steps);
            } else if WORLD[pos+1] == 1 { // Open cell
                goto(&Dir::R, pos, steps, all_steps);
            } else {
                got_stuck(pos);
            }
        }
        Dir::R => {
            all_steps.push(dir.clone());
            if WORLD[pos+1] == 1 {
                println!("R");
                steps.push(dir.clone());
                goto(dir, pos+1, steps, all_steps)
            } else if WORLD[pos-1] == 1 {
                goto(&Dir::L, pos, steps, all_steps);
            } else {                
                got_stuck(pos);
            }
        }
    };
}
