// Fully Qualified Syntax for Disambiguation:
// Calling Methods with the Same Name

// 1. 
trait Pilot {
    fn fly(&self) -> &str;
}
trait Wizard {
    fn fly(&self) -> &str;
}

struct Human;
impl Human {
    fn fly(&self) -> &str {
        "Human.fly"
    }
}
impl Pilot for Human {
    fn fly(&self) -> &str {
        "Pilot.fly"
    }
}
impl Wizard for Human {
    fn fly(&self) -> &str {
        "Wizard.fly"
    }
}

#[test] fn ex1() {
    let human = Human;
    assert_eq!(human.fly(), "Human.fly");
    assert_eq!(Pilot::fly(&human), "Pilot.fly");
    assert_eq!(Wizard::fly(&human), "Wizard.fly");
}


// 2. A trait with an associated function. A type with an associated
// function of the same name and also implements the trait

trait Animal {
    fn baby_name() -> String;
}

struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

#[test] fn ex2() {
    assert_eq!(Dog::baby_name(), "Spot");
    // assert_eq!(Animal::baby_name(), "Spot");
        // error[E0283]: type annotations needed, cannot infer type
    assert_eq!(<Dog as Animal>::baby_name(), "puppy");
        // Using fully qualified syntax to specify that we want to call the baby_name
        // function from the Animal trait as implemented on Dog
}