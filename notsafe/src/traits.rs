// the difference between generics and associated types is that
// with generics you can write implementations for different types while associated types require one type

use std::ops::Add;
use std::fmt;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

struct Millimeters(u32);
struct Meters(u32);

// trait definition, any type implementing pilot must provide a fly method
trait Pilot {
    fn fly(&self);

    fn chutzpah();
}

trait Wizard {
    fn fly(&self);
}

// specify when a trait depends on another trait, for a type to implement SpecialPrint it must first implement Display
trait SpecialPrint: fmt::Display {
    fn special(&self) {
        println!("{}", &self);
        println!("I'm relying on a supertrait");
    }
}

// struct without any fields because why not
struct Human;

// the orphan rule prevents us from implementing externally defined traits on externally defined types
// but we can use a tuple struct to get around this
struct Wrapper(Vec<String>);

// we aren't allowed to implement display on vec, but we can implement display on a type we define ourselves
// the issue is that wrapper is a new type and so doesn't have the methods that vec already has 
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("By the mechanical wizardry of post industrial society");
    }

    // we can't call this associated function because rust can't figure out whether to use the Pilot version or the normal version
    // associated functions don't have self parameters
    fn chutzpah() {
        println!("Panache");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("By the resonating frequencies of magical runes");
    }
}

impl Human {
    fn fly(&self) {
        println!("By sheer force of will");
    }

    fn chutzpah() {
        println!("Soldering iron");
    }
}

// define the behaviour for the + operator acting on two points
impl Add for Point {
    // Add has an associated type Output which determines the type of the returned by add
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// add meters and millimeters, the default type would be Millimeters but we've specified that this implementation is for meters
// Add has a generic type RHS which defaults to the type it's implemented for
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

pub fn trait_stuff() {
    let person = Human;
    // calling fly on a type that implements fly will call the corresponding method
    person.fly();
    // we can also specify which trait's fly method we want to call using more explicit syntax
    Pilot::fly(&person);
    // need to use fully qualified syntax to call the right chutzpah associated method
    <Human as Pilot>::chutzpah();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3}
        )
    }
}
