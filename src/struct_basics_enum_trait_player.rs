#![allow(dead_code)]

// interv-1

// skip? : this task is already being done in different tasks

// - create new Rust project
// - add player.rs into src/
// - struct Player - string name, position (x,y coord), life int, attack int
// - use auto derive Default and impl manually for Player
// - ctor w/ name param which accepts any type of strings, assigns to name and 
//   get others from default
// - fn name returns name
// - fn eat w/ param food which increases life not more than 99
// - fn lose_life w/ param val, life can be 0 minimum and dies in this case
// - fn attack_another_player, use attack power
// - fn move with params direction (X, Y coordinates) and val, to move across
//   X and Y directions
// - use player from main.rs
// - test fn eat





// -------------------------------------------------------------------





#[derive(Default)]
struct Position { x: i8, y: i8 } // x,y coordinates

enum Dir { X, Y }
// or
// enum Dir { Left, Right, Down, Up }

pub struct Player {
    name: String,
    pos: Position,
    life: u8,   // life cannot be more than 99
    attack: u8  // attack power
}

impl Player {
    fn name(&self) -> &String { &self.name }

    /// life cannot be more than 99
    pub fn eat(&mut self, food: u8) {
        let sum = self.life + food;
        if sum > 99 { self.life = 99; return; }
        self.life = sum;
    }

    pub fn lose_life(&mut self, val: u8) {
        if self.life <= val { // dies
            self.life = 0;
            return;
        }
        self.life -= val; 
    }

    pub fn attack_another_player(&self, p2: &mut Self) {
        p2.lose_life(self.attack); // p2 is damaged by self's attack
    }

    /// negative val means going down or left
    fn r#move(&mut self, dir: Dir, val: i8) {
        match dir {
            Dir::X => self.pos.x += val,
            Dir::Y => self.pos.y += val,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self { 
            name: "unnamed".to_string(),
            pos: Position::default(),
            life: 99,
            attack: 9
        }
    }
}

impl Player {
    // or Into<T>, AsRef<T> instead of ToString
    pub fn new_from_name(name: impl ToString) -> Self {
        Self { name: name.to_string(), ..Default::default() }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn attack_another_player() {
        let p1 = Player::default();
        let mut p2 = Player::default();
        assert_eq!(p1.life, 99);
        assert_eq!(p2.life, 99);
        p1.attack_another_player(&mut p2);
        assert_eq!(p2.life, 90);

    }

    #[test] fn lose_life() {
        let mut p = Player::default();
        assert_eq!(p.life, 99);
        p.lose_life(19);
        assert_eq!(p.life, 80);
    }

    #[test] fn eat() {
        let mut p1 = Player::default();
        assert_eq!(p1.life, 99);
        p1.eat(60);
        assert_eq!(p1.life, 99);
        p1.lose_life(9);
        assert_eq!(p1.life, 90);
    }

    #[test] fn r#move() {
        let mut p1 = Player::default();
        assert_eq!(p1.pos.x, 0);
        p1.r#move(Dir::X, 10);
        assert_eq!(p1.pos.x, 10);
        p1.r#move(Dir::X, -20);
        assert_eq!(p1.pos.x, -10);
    }
}