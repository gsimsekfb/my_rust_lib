
//// Calling trait and struct method and assoc. fns with same name

// 1
// Fully Qualified Syntax for Disambiguation:
// Calling Methods with the Same Name
//
// trait Pilot  with fn fly returns &str, def impl "Pilot.fly"
// trait Wizard with fn fly returns &str
// struct Human empty
// impl Human fn fly returns &str, def impl "Human.fly"
// impl Pilot and Wizard for Human
// Test:
// From a human obj, call these fly fns

// 1. 
trait Pilot { fn fly(&self) -> &str { "Pilot.fly" } } // todo: can we call this?
trait Wizard { fn fly(&self) -> &str; }

struct Human;
impl Human {
    fn fly(&self) -> &str { "Human.fly" }
}
impl Pilot for Human {
    fn fly(&self) -> &str { "Human as Pilot.fly" }
}
impl Wizard for Human {
    fn fly(&self) -> &str { "Human as Wizard.fly" }
}

#[test] fn ex1() {
    let human = Human;
    assert_eq!(human.fly(), "Human.fly");
    assert_eq!(Pilot::fly(&human), "Human as Pilot.fly");
        // or <Human as Pilot>::fly(&human)
    assert_eq!(Wizard::fly(&human), "Human as Wizard.fly");
}


// 2
// Calling trait and struct assoc. fns with the same Name
//
// trait Animal, assoc. fn name returns string "Animal"
// struct Dog, empty
// impl Dog assoc. fn name returns string "Dog"
// impl Animal for Dog
// Test:
// Call these 2 name fns

trait Animal { fn name() -> String { "Animal".to_string() } }

struct Dog;
impl Dog {
    fn name() -> String { "Dog".to_string() }
}
impl Animal for Dog {
    fn name() -> String { "Dog as Animal".to_string() }
}

#[test] fn ex2() {
    assert_eq!(Dog::name(), "Dog");
    // assert_eq!(Animal::name(), "Animal");
        // error[E0790]: cannot call associated function on trait without 
        // specifying the corresponding `impl` type
    assert_eq!(<Dog as Animal>::name(), "Dog as Animal");
        // Using fully qualified syntax to specify that we want to call the name
        // function from the Animal trait as implemented on Dog
}
