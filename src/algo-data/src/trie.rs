// interv-2
// last: 5/25

// Impl trie aka m-ary tree
// fn insert, see file: trie-Coding Interview_ Compressed Tries - HackMD.pdf
// fn height
// fn print
    // #
    // - c
    // - - a
    // - - - r
    // - - - - b
    // - - - - - o
    // - - - - - - n
    // - - - - - i
    // - - - - - - d
    // - - - - - - - e
    // - b
    // - - i
    // - - - b
    // - - - - l
    // - - - - - e
    // - - - - - i
    // - - - - - - c
    // - - - - - - - a
    // - - - - - - - - l

// ==========================================


// #[derive(Debug)]
struct Trie {
    key: char,
    val: Option<i32>,
    leaves: Vec<Trie>, // or HashMap<char, Trie> ?
}

impl Trie {

    fn insert(&mut self, word: &str, val: i32) {
        let Some(chr) = word.chars().next() else {
            return
        };
        match self.leaves.iter_mut().find(|leaf| leaf.key == chr) {
            Some(leaf_node) => { // leaf node w/ key=chr exists
                leaf_node.insert(&word[1..], val);
            },
            None => {  // leaf node w/ key=chr does not exist 
                let mut new_node = Trie { key: chr, val: None, leaves: vec![] };
                if word.len() == 1 {
                    new_node.val = Some(val);
                    self.leaves.push(new_node);
                } else {
                    self.leaves.push(new_node);
                    self.leaves.last_mut().unwrap().insert(&word[1..], val);
                }
            }
        }
    }

    fn height(&self) -> i32 {
        let mut max_h = -1;
        for leaf in &self.leaves {
            max_h = max_h.max(leaf.height());
        }
        1 + max_h
    }

    fn print(&self, prefix: &str) {
        println!("{}{}", prefix, &self.key);
        for leaf in &self.leaves {
            leaf.print(&[prefix, "- "].join(""));
        }
    }
}

#[test]
fn basics() {
    let mut t = Trie { key: '#', val: None, leaves: vec![] };
    t.insert("car", 11);
    assert_eq!(t.height(), 3);

    t.insert("carbon", 13);
    assert_eq!(t.height(), 6);

    t.insert("carbide", 15);
    assert_eq!(t.height(), 7);

    t.insert("bible", 17);
    assert_eq!(t.height(), 7);

    t.insert("biblical", 19);
    assert_eq!(t.height(), 8);
}


// ------------------------- main

fn main() {
    println!("---------");
    // Pinned

    let mut t = Trie { key: '#', val: None, leaves: vec![] };
    t.insert("car", 11);
    t.insert("carbon", 13);
    t.insert("carbide", 15);
    t.insert("bible", 17);
    t.insert("biblical", 19);

    t.print("");
    
    println!();
}

